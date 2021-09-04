/*!
 * Minimal theme switcher
 *
 * Pico.css - https://picocss.com
 * Copyright 2020 - Licensed under MIT
 */


const themeSwitcher = {
  // Config
  buttonsTarget: 'a[data-theme-switcher]',
  buttonAttribute: 'data-theme-switcher',
  rootAttribute: 'data-theme',

  set_theme(attribute) {
    document.querySelector('html')
      .setAttribute(
	this.rootAttribute,
	attribute
      );
    localStorage.setItem('theme', attribute);
  },

  // Init
  init() {
     switch(localStorage.getItem('theme')) {
	case 'light':
	 this.set_theme('light');
	 break;
	default:
	 this.set_theme('dark');
	 break;
     };

    document.querySelectorAll(this.buttonsTarget).forEach(function(button) {
      button.addEventListener('click', function(event) {
      event.preventDefault();
      this.set_theme(event.target.getAttribute(this.buttonAttribute));
      }.bind(this), false);
    }.bind(this));
  }
}

// Init
themeSwitcher.init();

