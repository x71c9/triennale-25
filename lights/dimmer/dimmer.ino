#include "DFRobot_GP8403.h"
#include <Wire.h>

DFRobot_GP8403 dac1(&Wire, 0x58);  // First DAC at address 0x58
DFRobot_GP8403 dac2(&Wire, 0x5F);  // Second DAC at address 0x5F

void setup() {
  Serial.begin(115200);

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

  dac1.setDACOutRange(dac1.eOutputRange10V);
  dac2.setDACOutRange(dac2.eOutputRange10V);

  // Set voltages on DAC1 (0x58)
  dac1.setDACOutVoltage(10000, 0);  // 10V on channel 0
  dac1.setDACOutVoltage(7500, 1);   // 7.5V on channel 1

  // Set voltages on DAC2 (0x5F)
  dac2.setDACOutVoltage(9000, 0);   // 9V on channel 0
  dac2.setDACOutVoltage(2000, 1);   // 2V on channel 1
}

void loop() {
  // Nothing needed here
}
