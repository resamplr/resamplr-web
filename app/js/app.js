import Elm from './main';
const elmDiv = document.querySelector('#elm-container');
if (elmDiv) {
  Elm.Main.embed(elmDiv);
}