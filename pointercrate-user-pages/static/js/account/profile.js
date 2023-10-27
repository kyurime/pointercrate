"use strict";

import {
  del,
  get,
  displayError,
  EditorBackend,
  Form,
  Output,
  post,
  setupEditorDialog,
  setupFormDialogEditor,
  tooShort,
  typeMismatch,
  valueMissing,
} from "/static/core/js/modules/form.js";

async function determinePasskeySupported() {
  // from <https://web.dev/articles/passkey-registration>
  if (window.PublicKeyCredential &&
    PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable &&
    PublicKeyCredential.isConditionalMediationAvailable) {
    const results = await Promise.all([
      PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable(),
      PublicKeyCredential.isConditionalMediationAvailable(),
    ]);

    return results.every(r => r === true);
  }

  return false;
}

function setupGetAccessToken() {
  var accessTokenArea = document.getElementById("token-area");
  var accessToken = document.getElementById("access-token");
  var getTokenButton = document.getElementById("get-token");

  var htmlLoginForm = document.getElementById("login-form");
  var loginForm = new Form(htmlLoginForm);

  getTokenButton.addEventListener(
    "click",
    () => {
      getTokenButton.style.display = "none";
      accessTokenArea.style.display = "none";
      htmlLoginForm.style.display = "block";
    },
    false
  );

  var loginPassword = loginForm.input("login-password");

  loginPassword.clearOnInvalid = true;
  loginPassword.addValidators({
    "Password required": valueMissing,
    "Password too short. It needs to be at least 10 characters long.": tooShort,
  });

  loginForm.onSubmit(function () {
    post("/api/v1/auth/", {
      Authorization:
        "Basic " + btoa(window.username + ":" + loginPassword.value),
    })
      .then((response) => {
        loginPassword.value = "";
        accessToken.innerHTML = response.data.token;
        htmlLoginForm.style.display = "none";
        accessTokenArea.style.display = "block";
      })
      .catch((response) => {
        if (response.data.code == 40100) {
          loginPassword.errorText = "Invalid credentials";
        } else {
          loginForm.setError(response.data.message);
        }
      });
  });
}

class ProfileEditorBackend extends EditorBackend {
  constructor(passwordInput) {
    super();

    this._pw = passwordInput;
    this._displayName = document.getElementById("profile-display-name");
    this._youtube = document.getElementById("profile-youtube-channel");
  }

  url() {
    return "/api/v1/auth/me/";
  }

  headers() {
    return {
      "If-Match": window.etag,
      Authorization: "Basic " + btoa(window.username + ":" + this._pw.value),
    };
  }

  onSuccess(response) {
    if (response.status == 204) {
      window.location.reload();
    } else {
      window.etag = response.headers["etag"];
      window.username = response.data.data.name;

      this._displayName.innerText = response.data.data.display_name || "None";
      this._youtube.removeChild(this._youtube.lastChild); // only ever has one child
      if (response.data.data.youtube_channel) {
        let a = document.createElement("a");
        a.href = response.data.data.youtube_channel;
        a.classList.add("link");
        this._youtube.appendChild(a);
      } else {
        this._youtube.innerText = "-";
      }
    }
  }
}

function setupEditAccount() {
  let output = new Output(document.getElementById("things"));

  let editDisplayNameForm = setupFormDialogEditor(
    new ProfileEditorBackend(document.querySelector("#auth-dn input")), // not pretty, but oh well
    "edit-dn-dialog",
    "display-name-pen",
    output
  );

  editDisplayNameForm.addValidators({
    "auth-dn": {
      "Password required": valueMissing,
      "Password too short. It needs to be at least 10 characters long.": tooShort,
    },
  });

  editDisplayNameForm.addErrorOverride(40100, "auth-dn");

  let editYoutubeForm = setupFormDialogEditor(
    new ProfileEditorBackend(document.querySelector("#auth-yt input")), // not pretty, but oh well
    "edit-yt-dialog",
    "youtube-pen",
    output
  );

  editYoutubeForm.addValidators({
    "edit-yt": {
      "Please enter a valid URL": typeMismatch,
    },
    "auth-yt": {
      "Password required": valueMissing,
      "Password too short. It needs to be at least 10 characters long.": tooShort,
    },
  });

  editYoutubeForm.addErrorOverride(40100, "auth-yt");
  editYoutubeForm.addErrorOverride(42225, "edit-yt");

  /*let changeEmailForm = setupFormDialogEditor(
      new ProfileEditorBackend(document.querySelector("#auth-email input")), // not pretty, but oh well
      "edit-email-dialog",
      "email-pen",
      output
  );

  changeEmailForm.addValidators({
    "edit-email": {
      "Please enter a valid e-mail address": typeMismatch,
    },
    "auth-email": {
      "Password required": valueMissing,
      "Password too short. It needs to be at least 10 characters long.": tooShort,
    },
  });

  changeEmailForm.addErrorOverride(40100, "auth-email");*/

  let changePasswordForm = setupFormDialogEditor(
    new ProfileEditorBackend(document.querySelector("#auth-pw input")), // not pretty, but oh well
    "edit-pw-dialog",
    "change-password",
    output
  );

  let editPw = changePasswordForm.input("edit-pw");

  changePasswordForm.addValidators({
    "auth-pw": {
      "Password required": valueMissing,
      "Password too short. It needs to be at least 10 characters long.": tooShort,
    },
    "edit-pw": {
      "Password too short. It needs to be at least 10 characters long.": tooShort,
    },
    "edit-pw-repeat": {
      "Password too short. It needs to be at least 10 characters long.": tooShort,
      "Passwords don't match": (rpp) => rpp.value == editPw.value,
    },
  });

  changePasswordForm.addErrorOverride(40100, "auth-pw");

  var deleteAccountDialog = document.getElementById("delete-acc-dialog");
  var deleteAccountForm = new Form(
    deleteAccountDialog.getElementsByTagName("form")[0]
  );
  document.getElementById("delete-account").addEventListener("click", () => {
    $(deleteAccountDialog.parentElement).show();
  });

  var deleteAuth = deleteAccountForm.input("auth-delete");
  deleteAuth.addValidators({
    "Password required": valueMissing,
    "Password too short. It needs to be at least 10 characters long.": tooShort,
  });

  deleteAccountForm.addErrorOverride(40100, "auth-delete");

  deleteAccountForm.onSubmit(() => {
    del("/api/v1/auth/me/", {
      "If-Match": window.etag,
      Authorization: "Basic " + btoa(window.username + ":" + deleteAuth.value),
    })
      .then(() => window.location.reload())
      .catch(displayError(deleteAccountForm));
  });

  var addPasskeyDialog = document.getElementById("add-passkey-dialog");
  var addPasskeyForm = new Form(
    addPasskeyDialog.getElementsByTagName("form")[0]
  );

  document.getElementById("add-passkey").addEventListener("click", () => {
    $(addPasskeyDialog.parentElement).show();
  });

  var passkeyTitle = addPasskeyForm.input("passkey-name");
  passkeyTitle.addValidators({
    "Passkey title required": valueMissing,
  });

  addPasskeyForm.onSubmit(async () => {
    if (!(await determinePasskeySupported())) {
      return displayError(addPasskeyForm)({
        data: {
          message: "Your browser does not support this feature!", code: 10010
        }
      });
    }

    const get_req = await get("/api/v1/auth/me/");

    const profile_data = get_req.data.data;

    try {
      const credential_options = {
        challenge: new Uint8Array(), // this field supposedly goes unused
        rp: {
          name: "1.9 GDPS Demonlist",
          id: "localhost",
        },
        user: {
          id: Uint8Array.from(profile_data.id),
          name: profile_data.name,
          displayName: profile_data.display_name,
        },
        pubKeyCredParams: [{
          alg: -7, type: "public-key"
        }, {
          alg: -257, type: "public-key"
        }],
  /*
        excludeCredentials: [{
          id: *****,
          type: 'public-key',
          transports: ['internal'],
        }],
  */
        authenticatorSelection: {
          requireResidentKey: true,
        }
      };
  
      console.log(credential_options);
  
      const credential = await navigator.credentials.create({
        publicKey: credential_options
      });
  
      console.log(credential);
    } catch (e) {
      return displayError(addPasskeyForm)(e);
    }
/*
    del("/api/v1/auth/me/", {
      "If-Match": window.etag,
      Authorization: "Basic " + btoa(window.username + ":" + deleteAuth.value),
    })
      .then(() => window.location.reload())
      .catch(displayError(deleteAccountForm));
*/
  });
}

function setupInvalidateToken() {
  var invalidateButton = document.getElementById("invalidate-token");
  var htmlInvalidateForm = document.getElementById("invalidate-form");
  var invalidateForm = new Form(htmlInvalidateForm);

  invalidateButton.addEventListener(
    "click",
    () => {
      invalidateButton.style.display = "none";
      htmlInvalidateForm.style.display = "block";
    },
    false
  );

  var invalidatePassword = invalidateForm.input("invalidate-auth-password");

  invalidatePassword.clearOnInvalid = true;
  invalidateForm.addValidators({
    "invalidate-auth-password": {
      "Password required": valueMissing,
      "Password too short. It needs to be at least 10 characters long.": tooShort,
    },
  });

  invalidateForm.addErrorOverride(40100, "invalidate-auth-password");

  invalidateForm.onSubmit(function () {
    post("/api/v1/auth/invalidate/", {
      Authorization:
        "Basic " + btoa(window.username + ":" + invalidatePassword.value),
    })
      .then(() => window.location.reload())
      .catch(displayError(invalidateForm));
  });
}

export async function initialize() {
  await determinePasskeySupported();

  setupGetAccessToken();
  setupEditAccount();
  setupInvalidateToken();
}
