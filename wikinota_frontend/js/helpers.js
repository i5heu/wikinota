function Sleep(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

class InputNotice {

  static get inputNotice() {
    return document.getElementById("inputNotice");
  }

  static hide(){
    document.body.classList.remove("inputNoticeActive");
  }

  static set content(val) {
    this.inputNotice.innerHTML = val;
    document.body.classList.add("inputNoticeActive");
  }

  static get content() {
    return this.inputNotice.innerHTML;
  }
}