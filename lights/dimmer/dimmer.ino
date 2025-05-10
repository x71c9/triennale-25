#include "DFRobot_GP8403.h"
#include <Wire.h>

DFRobot_GP8403 dac1(&Wire, 0x58);  // First DAC at address 0x58
DFRobot_GP8403 dac2(&Wire, 0x5F);  // Second DAC at address 0x5F
DFRobot_GP8403 dac3(&Wire, 0x5A);  // Third DAC at address 0x5A

int dac_values[6] = {
  1111, // Channel 1
  2222, // Channel 2
  3333, // Channel 3
  4444, // Channel 4
  5555, // Channel 5
  6666  // Channel 6
};

void setup() {
  Serial.begin(115200);
  while (!Serial);  // Wait for serial to be ready

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

  // Initialize all 6 outputs
  updateAllDACOutputs();
}

void updateAllDACOutputs() {
  dac1.setDACOutVoltage(dac_values[0], 0); // CH1
  dac1.setDACOutVoltage(dac_values[1], 1); // CH2
  dac2.setDACOutVoltage(dac_values[2], 0); // CH3
  dac2.setDACOutVoltage(dac_values[3], 1); // CH4
  dac3.setDACOutVoltage(dac_values[4], 0); // CH5
  dac3.setDACOutVoltage(dac_values[5], 1); // CH6
}

void loop() {
  if (Serial.available()) {
    String input = Serial.readStringUntil('\n');
    input.trim();
    input.toUpperCase();

    Serial.print("Received: ");
    Serial.println(input);

    if (input.startsWith("DIM")) {
      int channel = input.substring(4, 5).toInt();
      int value = input.substring(6).toInt();

      // Clamp to safe range
      value = constrain(value, 0, 10000);

      if (channel < 1 || channel > 6) {
        Serial.println("Invalid channel. Use 1–6.");
        return;
      }

      dac_values[channel - 1] = value;

      Serial.print("Parsed channel: ");
      Serial.print(channel);
      Serial.print(", value: ");
      Serial.println(value);

      // Apply all 6 values every time
      updateAllDACOutputs();

      Serial.println("All DAC values updated.");
    } else {
      Serial.println("Invalid command. Use: DIM <1–6> <0–10000>");
    }
  }
}
