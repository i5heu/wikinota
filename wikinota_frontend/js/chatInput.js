class chatInput {
  inputEl;

  constructor() {
    this.inputEl = window.mainInput
    this.inputEl.addEventListener("keypress", ev => this.mainInputEventHandler(ev));
  }

  /**
   * will handle the event of the #mainInput
   * @param {Event} ev 
   */
  mainInputEventHandler(ev) {
    console.log(ev);

    switch (ev.key) {
      case "Enter":
        this.enterPath(ev);
        break;

      default:
        break;
    }
  }

  returnValueFromNextEnter(){
    
  }

  enterPath(ev) {
    const inputVal = this.inputEl.value;
    if (inputVal.charAt(0) == "/") {
      masterLogic.commandManager(inputVal);
    }
  }
}