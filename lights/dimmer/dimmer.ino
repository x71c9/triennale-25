const int pin5 = 5;
const int pin6 = 6;

float analogValue = 0.0;
const float step = 0.2;
const unsigned long interval = 10000; // 10 seconds in milliseconds
unsigned long lastUpdate = 0;

void setup() {
  pinMode(pin5, OUTPUT);
  pinMode(pin6, OUTPUT);
}

void loop() {
  unsigned long currentMillis = millis();

  if (currentMillis - lastUpdate >= interval) {
    lastUpdate = currentMillis;

    // Map value (0.0 to 1.0) to PWM range (0 to 255)
    int pwmValue = int(analogValue * 255);
    analogWrite(pin5, pwmValue);
    analogWrite(pin6, pwmValue);

    // Increment analogValue
    analogValue += step;
    if (analogValue > 1.0) {
      analogValue = 0.0; // Reset after reaching 1
    }
  }
}
