class uiSugar {
	constructor() {
		this.initUiClocks();
	}

	async initUiClocks() {
		while (true) {
			const date = new Date();

			var options = { timeZoneName: "short" };
			document.getElementById("clock-local").innerText = date.toLocaleString(
				"de-de",
				options
			);

			var optionsUtc = { timeZone: "UTC", timeZoneName: "short" };
			document.getElementById("clock-UTC").innerText = date.toLocaleString(
				"de-de",
				optionsUtc
			);

			await Sleep(250);
		}
	}
}