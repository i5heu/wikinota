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
	static async login(){
		const promToken = serverInteraction.getLoginToken();
		window.mainInput.value = "";
		InputNotice.content = "Input Username";

		let username, password = "";
		// get username
		// get password
		processes.login(username, password);
	}
}

class processes {
	static async login(username, password, promiseLoginToken){
		// genrate random hash 20digits long
		// save hash (credentials + 20digits hash) as credentailStorehash
		const sessionToken = await promiseLoginToken;
		// generate sessionHash from sessionToken and credentailStorehash
		// send sessionHash solution
		// save sessionHash in sessionStorage
		await serverInteraction.getSessionToken();
		// save sessionToken
	}
}

class serverInteraction {
	static async getLoginToken(){

	}

	static async getSessionToken(sessionHash){
	}
}