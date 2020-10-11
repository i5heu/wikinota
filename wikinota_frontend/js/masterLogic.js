class masterLogic {
  /**
   * will find and execute an command by string
   * @param {string} command
   */
  static commandManager(command) {
    console.log("COM > ", command);
    switch (command) {
      case "/login":
        uiProcceses.login();
        break;

      default:
        break;
    }
  }
}

class uiProcceses {
  static async login() {
    window.mainInput.value = "";

    const loginModal = document.getElementById("loginModal");
    loginModal.style.display = "block";

    loginModal.querySelector("form").addEventListener("submit", async (ev) => {
      ev.preventDefault();
      const usernameEl = loginModal.querySelector("#username");
      const passwordEl = loginModal.querySelector("#password");

      const status = await processes.login(usernameEl.value, passwordEl.value);
      console.log("loginStatus =", status);
      if (status) {
        this.uiFlashMessage("Login was Succesfull :)", "success");
        document.getElementById("loginModal").style.display = "none";
      } else {
        this.uiFlashMessage("Login was not Successfull :(", "error");
      }
    });
  }

  static uiFlashMessage(message = "no message", type = "default") {
    const flashMessageContainer = document.getElementById(
      "flashMessageContainer"
    );

    const flashMessage = document.createElement("div");
    flashMessage.className = "flashMessage " + type;
    flashMessage.innerHTML = message;

    flashMessageContainer.appendChild(flashMessage);

    flashMessage.addEventListener("pointerup", () => flashMessage.remove());

    setTimeout(() => {
      if (flashMessage) flashMessage.remove();
    }, 1500);
  }
}

class processes {
  static async login(username, password) {
    try {
      const sessionToken = await serverInteraction.getLoginToken();

      console.log("sessionToken", sessionToken);

      if (!sessionToken) {
        uiProcceses.uiFlashMessage("LoginToken was not provided", "error");
        return false;
      }
      await serverInteraction.getSessionToken();

      return false;
    } catch (error) {
      uiProcceses.uiFlashMessage("Err LoginToken > " + error.message, "error");
    }
  }
}

class serverInteraction {
  static async getLoginToken() {
    const response = await fetch("/api/loginToken");
    const loginData = await response.json();
    return loginData.loginToken;
  }

  static async getSessionToken(sessionHash) {}
}
