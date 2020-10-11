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
		const promToken = serverInteraction.getLoginToken();
		window.mainInput.value = "";

		const loginModal = document.getElementById("loginModal");
		loginModal.style.display = "block";

		loginModal.querySelector("form").addEventListener("submit", async ev => {
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
		const flashMessageContainer = document.getElementById("flashMessageContainer");

		const flashMessage = document.createElement("div");
		flashMessage.className = "flashMessage " + type;
		flashMessage.innerHTML = message;

		flashMessageContainer.appendChild(flashMessage);

		flashMessage.addEventListener("pointerup", () => flashMessage.remove());

		setTimeout(() => {
			if(flashMessage) flashMessage.remove();
		}, 1500);
	}
}

class processes {
	static async login(username, password) {
		// genrate random hash 20digits long
		// save hash (credentials + 20digits hash) as credentailStorehash
		const sessionToken = await promiseLoginToken();
		// generate sessionHash from sessionToken and credentailStorehash
		// send sessionHash solution
		// save sessionHash in sessionStorage
		await serverInteraction.getSessionToken();
		// save sessionToken

		return false;
	}
}

class serverInteraction {
	static async getLoginToken() {

	}

	static async getSessionToken(sessionHash) {
	}
}