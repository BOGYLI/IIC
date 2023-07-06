// Array mit den verschiedenen Sound-Dateien
var sounds = ["autumn-beach-sound-6084.mp3", "gentle-ocean-waves-birdsong-and-gull-7109.mp3", "calm-river-ambience-loop-125071.mp3"];

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
