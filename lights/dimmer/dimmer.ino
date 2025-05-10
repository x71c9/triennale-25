#include "DFRobot_GP8403.h"
#include <Wire.h>

DFRobot_GP8403 dac1(&Wire, 0x58);  // First DAC at address 0x58
DFRobot_GP8403 dac2(&Wire, 0x5F);  // Second DAC at address 0x5F
DFRobot_GP8403 dac3(&Wire, 0x5A);  // Second DAC at address 0x5F

int voltageStep = 0;

void setup() {

  Serial.begin(115200);
  while (!Serial);

  while (dac1.begin() != 0) {
    Serial.println("DAC1 (0x58) init error");
    delay(1000);
  }
  Serial.println("DAC1 (0x58) init OK");

  while (dac2.begin() != 0) {
    Serial.println("DAC2 (0x5F) init error");
    delay(1000);
  }
  Serial.println("DAC2 (0x5F) init OK");

  while (dac3.begin() != 0) {
    Serial.println("DAC3 (0x5A) init error");
    delay(1000);
  }
  Serial.println("DAC3 (0x5A) init OK");

  dac1.setDACOutRange(dac1.eOutputRange10V);
  dac2.setDACOutRange(dac2.eOutputRange10V);
  dac3.setDACOutRange(dac3.eOutputRange10V);

  // Set voltages on DAC1 (0x58)
  dac1.setDACOutVoltage(1000, 0);  // 1V on channel 0
  dac1.setDACOutVoltage(7500, 1);   // 7.5V on channel 1

  // Set voltages on DAC2 (0x5F)
  dac2.setDACOutVoltage(9000, 0);   // 9V on channel 0
  dac2.setDACOutVoltage(2000, 1);   // 2V on channel 1

  // Set voltages on DAC3 (0x5A)
  dac3.setDACOutVoltage(6767, 0);   // 6.76V on channel 0
  dac3.setDACOutVoltage(1212, 1);   // 1.21V on channel 1

}

void loop() {
  if (Serial.available()) {
    String input = Serial.readStringUntil('\n');
    input.trim();  // remove leading/trailing whitespace
    input.toUpperCase();

    Serial.print("Received: ");
    Serial.println(input);

    if (input.startsWith("DIM")) {
      int channel = input.substring(4, 5).toInt();
      int value = input.substring(6).toInt();

      // Clamp the voltage value between 0 and 10000
      value = constrain(value, 0, 10000);

      Serial.print("Parsed channel: ");
      Serial.print(channel);
      Serial.print(", value: ");
      Serial.println(value);

      switch (channel) {
        case 1: dac1.setDACOutVoltage(value, 0); break;
        case 2: dac1.setDACOutVoltage(value, 1); break;
        case 3: dac2.setDACOutVoltage(value, 0); break;
        case 4: dac2.setDACOutVoltage(value, 1); break;
        case 5: dac3.setDACOutVoltage(value, 0); break;
        case 6: dac3.setDACOutVoltage(value, 1); break;
        default:
          Serial.println("Invalid channel. Use 1–6.");
          return;
      }

      Serial.println("DAC value set successfully.");
    } else {
      Serial.println("Invalid command. Use: DIM <1–6> <0–10000>");
    }
  }
}
