// Array mit den verschiedenen Sound-Dateien
var sounds = ["sound1.mp3", "sound2.mp3", "sound3.mp3"];

// Funktion, um einen zufälligen Sound abzuspielen
function playRandomSound() {
  // Zufällige Indexnummer auswählen
  var randomIndex = Math.floor(Math.random() * sounds.length);
  
  // Sound-Datei abspielen
  var audio = new Audio(sounds[randomIndex]);
  audio.play();
}

// Funktion aufrufen, um einen zufälligen Sound abzuspielen
playRandomSound();
