import {
  initializeRecordSubmitter, initializeTimeMachine,
} from "/static/demonlist/js/modules/demonlist.js";
import {get} from "/static/core/js/modules/form.js";

$(document).ready(function () {
  if(window.demon_id) {
    initializeHistoryTable();
  }

  initializeRecordSubmitter();
  initializeTimeMachine();
});

function initializeHistoryTable() {
  if (!window.demon_id) {
    return;
  }

  get("/api/v2/demons/" + window.demon_id + "/audit/movement/").then(response => {
    let data = response.data;
    let tableBody = document.getElementById("history-table-body");

    let lastPosition = null;

    for (const entry of data) {
      let newRow = document.createElement("tr");
      let cells = [1, 2, 3, 4].map(() => document.createElement("td"));

      if (entry["new_position"] > window.extended_list_length && lastPosition > window.extended_list_length) {
        // skip movements that happen completely on the legacy list
        continue;
      }

      cells[0].innerText = entry["time"].split("T")[0];
      cells[0].style.whiteSpace = "nowrap";

      let positionChange = entry["new_position"] - lastPosition;

      if (lastPosition !== null) {
        let arrow = document.createElement("i");

        if (positionChange < 0) {
          arrow.classList.add("fas", "fa-arrow-up");
          newRow.classList.add("moved-up");
        } else {
          arrow.classList.add("fas", "fa-arrow-down");
          newRow.classList.add("moved-down");
        }

        if(entry["new_position"] > window.extended_list_length || lastPosition > window.extended_list_length) {
          cells[3].appendChild(document.createTextNode("Legacy"));
        } else {
          cells[3].appendChild(arrow);
          cells[3].appendChild(document.createTextNode(" " + Math.abs(positionChange)));
        }
      } else {
        cells[3].innerText = "-";
      }

      if (entry["new_position"] !== undefined) {
        if(entry["new_position"] > window.extended_list_length)
          cells[2].innerText = "-";
        else
        cells[2].innerText = entry["new_position"];
      }

      let reason = null;

      if(entry["reason"] === "Added") {
        reason = "Added to list";
      } else if(entry["reason"] === "Moved") {
        reason = "Moved";
      } else {
        if(entry["reason"]["OtherAddedAbove"] !== undefined) {
          let other = entry["reason"]["OtherAddedAbove"]["other"];
          let name = other.name === null ? "A demon" : other["name"];

          name = name.length > 24 ? `${name.substring(0, 24)}...` : name;

          reason = name + " was added above";

        } else if (entry["reason"]["OtherMoved"] !== undefined) {
          let other = entry["reason"]["OtherMoved"]["other"];
          let verb = positionChange < 0 ? "down" : "up";
          let name = other.name === null ? "A demon" : other["name"];

          name = name.length > 24 ? `${name.substring(0, 24)}...` : name;

          reason = name + " was moved " + verb + " past this demon"
        }
      }

      cells[1].innerText = reason;

      lastPosition = entry["new_position"];

      cells.forEach(cell => newRow.appendChild(cell));
      tableBody.appendChild(newRow);
    }
  });
}
