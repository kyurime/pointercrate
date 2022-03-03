import {
  Dropdown,
  Form,
  post,
  valueMissing,
  typeMismatch,
  badInput,
  stepMismatch,
  rangeUnderflow,
  rangeOverflow,
  tooLong,
  findParentWithClass,
  FilteredPaginator,
  Viewer,
  setupFormDialogEditor, FormDialog, setupEditorDialog, get,
} from "/static/core/js/modules/form.js";

export function embedVideo(video) {
  if (!video) return;
  // welcome to incredibly fragile string parsing with stadust
  // see pointercrate::video::embed for a proper implementation of this

  if (video.startsWith("https://www.youtube")) {
    return "https://www.youtube.com/embed/" + video.substring(32);
  }

  if (video.startsWith("https://www.twitch")) {
    return (
      "https://player.twitch.tv/?autoplay=false&parent=pointercrate.com&video=" +
      video.substring(29)
    );
  }
}

export function initializeTimeMachine() {
  let formHtml = document.getElementById("time-machine-form");

  if(formHtml === null)
    return;

  var timeMachineForm = new Form(formHtml);

  var inputs = ['year', 'month', 'day', 'hour', 'minute', 'second'].map(name => timeMachineForm.input("time-machine-" + name));

  for(let input of inputs) {
    input.addValidator(input => input.dropdown.selected !== undefined, "Please specify a value");
  }

  var offset = new Date().getTimezoneOffset();
  var offsetHours = Math.abs(offset) / 60;
  var offsetMinutes = Math.abs(offset) % 60;

  const MONTHS  = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];

  timeMachineForm.onSubmit(() => {
    let when = inputs[0].value + "-"
        + ("" + (MONTHS.indexOf(inputs[1].value) + 1)).padStart(2, '0') + "-"
        + ("" + inputs[2].value).padStart(2, '0') + "T"
        + ("" + inputs[3].value).padStart(2, '0') + ":"
        + ("" + inputs[4].value).padStart(2, '0') + ":"
        + ("" + inputs[5].value).padStart(2, '0') + (offsetHours < 0 ? "%2B" : "-") + (offsetHours + "").padStart(2, "0") + ":" + (offsetMinutes + "").padStart(2, "0");

    document.cookie = "when=" + when;
    gtag('event', 'time-machine-usage', {'event-category': 'demonlist', 'label': when});

    window.location = "/demonlist/";
  })
}

export function initializeRecordSubmitter(csrf = null, submitApproved = false) {
  var submissionForm = new Form(document.getElementById("submission-form"));

  var demon = submissionForm.input("id_demon");
  var player = submissionForm.input("id_player");
  var progress = submissionForm.input("id_progress");
  var video = submissionForm.input("id_video");
  var rawFootage = submissionForm.input("submit-raw-footage");

  demon.addValidator(input => input.dropdown.selected !== undefined, "Please specify a demon");

  let holderSelector = new PlayerSelectionDialog("submission-holder-dialog");
  document.getElementById("record-submitter-holder-pen").addEventListener('click', () => holderSelector.open().then(data => player.value = data.player));

  player.addValidator(input => input.value !== undefined, "Please specify a record holder");
  player.addValidator(
    input => input.value === undefined || input.value.length <= 50,
    "Due to Geometry Dash's limitations I know that no player has such a long name"
  );

  progress.addValidator(valueMissing, "Please specify the record's progress");
  progress.addValidator(rangeUnderflow, "Record progress cannot be negative");
  progress.addValidator(
    rangeOverflow,
    "Record progress cannot be larger than 100%"
  );
  progress.addValidator(badInput, "Record progress must be a valid integer");
  progress.addValidator(stepMismatch, "Record progress mustn't be a decimal");

  video.addValidator(
    valueMissing,
    "Please specify a video so we can check the records validity"
  );
  video.addValidator(typeMismatch, "Please enter a valid URL");

  rawFootage.addValidator(typeMismatch, "Please enter a valid URL");

  submissionForm.onInvalid(() => gtag('event', 'record-submit-failure-frontend', {'event-category': 'demonlist'}));
  submissionForm.onSubmit(function () {
    let data = submissionForm.serialize();
    let headers = {};

    if (submitApproved) {
      data.status = "approved";
      headers["X-CSRF-TOKEN"] = csrf;
    }
    post("/api/v1/records/", headers, data)
      .then(() => {
        submissionForm.setSuccess("Record successfully submitted");
        submissionForm.clear();
        gtag('event', 'record-submit-success', {'event-category': 'demonlist'});
      })
      .catch((response) =>  {
        switch(response.data.code) {
          case 40401:
            demon.errorText = response.data.message;
            break;
          case 42218:
            player.errorText = response.data.message;
            break;
          case 42215:
          case 42220:
            progress.errorText = response.data.message;
            break;
          case 42222:
          case 42223:
          case 42224:
          case 42225:
            video.errorText = response.data.message;
            break;
          case 42232:
          case 42233:
            rawFootage.errorText = response.data.message;
            break;
          default:
            submissionForm.setError(response.data.message)
        }
        gtag('event', 'record-submit-failure-backend', {'event-category': 'demonlist'});
      }); // TODO: maybe specially handle some error codes
  });
}

export function getCountryFlag(title, countryCode) {
  let countrySpan = document.createElement("span");
  countrySpan.classList.add("flag-icon");
  countrySpan.title = title;
  countrySpan.style.backgroundImage = "url(/static/demonlist/images/flags/" + countryCode.toLowerCase() + ".svg";
  return countrySpan;
}

export function getSubdivisionFlag(title, countryCode, subdivisionCode) {
  let stateSpan = document.createElement("span");
  stateSpan.classList.add("flag-icon");
  stateSpan.title = title;
  stateSpan.style.backgroundImage = "url(/static/demonlist/images/flags/" + countryCode.toLowerCase() + "/" + subdivisionCode.toLowerCase() + ".svg";
  return stateSpan;
}

export function populateSubdivisionDropdown(dropdown, countryCode) {
  dropdown.clearOptions();

  return get("/api/v1/nationalities/" + countryCode + "/subdivisions/").then(result => {
    for(let subdivision of result.data) {
      let flag = getSubdivisionFlag(subdivision.name, countryCode, subdivision.iso_code);

      flag.style.marginLeft = "-10px";
      flag.style.paddingRight = "1em";

      let li = document.createElement("li");

      li.className = "white hover";
      li.dataset.value = subdivision.iso_code;
      li.dataset.display = subdivision.name;
      li.appendChild(flag);
      li.appendChild(document.createTextNode(subdivision.name));

      dropdown.addLI(li);
    }
  });
}

export class PlayerSelectionDialog extends FormDialog {
  constructor(dialogId, selectionHandler) {
    super(dialogId);

    let paginator = new FilteredPaginator(
        dialogId + "-pagination",
        generatePlayer,
        "name_contains"
    );

    let playerName = this.form.inputs[0];

    playerName.addValidator(valueMissing, "Please provide a player name");

    paginator.initialize();
    if(selectionHandler === undefined) {
      paginator.addSelectionListener((selected) => {
        playerName.value = selected.name;
        this.form.html.requestSubmit();
      });
    } else {
      paginator.addSelectionListener(selectionHandler);
    }
  }
}

export function generatePlayer(player) {
  var li = document.createElement("li");
  var b = document.createElement("b");
  var b2 = document.createElement("b");

  li.className = "dark-gray";

  if (player.banned) {
    li.style.borderLeftColor = "rgba(221, 54, 78, .8)";
  } else {
    li.style.borderLeftColor = "rgba(164, 253, 106, .8)";
  }

  li.style.borderLeftWidth = "4px";
  li.style.borderLeftStyle = "solid";
  li.style.paddingLeft = "0.75em";

  li.dataset.name = player.name;
  li.dataset.id = player.id;

  b2.appendChild(document.createTextNode(player.id));

  if (player.nationality) {
    var span = document.createElement("span");

    span.className =
      "flag-icon flag-icon-" + player.nationality.country_code.toLowerCase();

    li.appendChild(span);
    li.appendChild(document.createTextNode(" "));
  }

  li.appendChild(b);
  li.appendChild(document.createTextNode(player.name + " - "));
  li.appendChild(b2);

  return li;
}

export function generateDemon(demon) {
  let li = document.createElement("li");
  let b = document.createElement("b");

  li.dataset.id = demon.id;

  b.innerText = "#" + demon.position + " - ";

  li.appendChild(b);
  li.appendChild(
    document.createTextNode(demon.name + " (ID: " + demon.id + ")")
  );
  li.appendChild(document.createElement("br"));
  li.appendChild(document.createTextNode("by " + demon.publisher.name));

  return li;
}

export function generateRecord(record) {
  var li = document.createElement("li");
  var recordId = document.createElement("b");

  li.className = "dark-gray";
  li.dataset.id = record.id;

  switch (record.status) {
    case "approved":
      li.style.borderLeftColor = "rgba(164, 253, 106, .8)";
      break;
    case "rejected":
      li.style.borderLeftColor = "rgba(221, 54, 78, .8)";
      break;
    case "submitted":
      li.style.borderLeftColor = "rgba(255, 255, 0, 1)";
      break;
    case "under consideration":
      li.style.borderLeftColor = "rgba(142, 230, 230, .8)";
      break;
    default:
      break;
  }

  li.style.borderLeftWidth = "4px";
  li.style.borderLeftStyle = "solid";
  li.style.paddingLeft = "0.75em";

  recordId.appendChild(document.createTextNode("Record #" + record.id));

  li.appendChild(recordId);
  li.appendChild(document.createElement("br"));
  li.appendChild(
    document.createTextNode(record.player.name + " (" + record.player.id + ")")
  );
  li.appendChild(document.createElement("br"));
  li.appendChild(
    document.createTextNode(record.progress + "% on " + record.demon.name)
  );
  li.appendChild(document.createElement("br"));

  return li;
}