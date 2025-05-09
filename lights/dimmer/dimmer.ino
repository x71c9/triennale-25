#include "DFRobot_GP8403.h"
#include <Wire.h>

DFRobot_GP8403 dac1(&Wire, 0x58);  // First DAC at address 0x58
// DFRobot_GP8403 dac2(&Wire, 0x5F);  // Second DAC at address 0x5F

int voltageStep = 0;

void setup() {
  Serial.begin(115200);

  while (dac1.begin() != 0) {
    Serial.println("DAC1 (0x58) init error");
    delay(1000);
  }
  Serial.println("DAC1 (0x58) init OK");

  // while (dac2.begin() != 0) {
  //   Serial.println("DAC2 (0x5F) init error");
  //   delay(1000);
  // }
  // Serial.println("DAC2 (0x5F) init OK");

  dac1.setDACOutRange(dac1.eOutputRange10V);
  // dac2.setDACOutRange(dac2.eOutputRange10V);

  // Set voltages on DAC1 (0x58)
  dac1.setDACOutVoltage(1000, 0);  // 1V on channel 0
  // dac1.setDACOutVoltage(7500, 1);   // 7.5V on channel 1

  // Set voltages on DAC2 (0x5F)
  // dac2.setDACOutVoltage(9000, 0);   // 9V on channel 0
  // dac2.setDACOutVoltage(2000, 1);   // 2V on channel 1
}

void loop() {

  int millivolts = voltageStep * 1000;  // Convert to mV
  dac1.setDACOutVoltage(millivolts, 0);
  Serial.print("Set DAC1 channel 0 to ");
  Serial.print(millivolts);
  Serial.println(" mV");

  // Optional: keep channel 1 of DAC1 at fixed 7.5V
  // dac1.setDACOutVoltage(7500, 1);

  voltageStep++;
  if (voltageStep > 10) {
    voltageStep = 0;  // Reset to 0 after reaching 10V
  }

  delay(4000);  // Wait 4 seconds before next step
}
