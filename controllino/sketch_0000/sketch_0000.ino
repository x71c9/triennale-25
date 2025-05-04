#include <SPI.h>
#include <Ethernet.h>
#include <EthernetServer.h>
#include <Controllino.h> // Include CONTROLLINO library

// Network settings
byte mac[] = { 0xDE, 0xAD, 0xBE, 0xEF, 0xFE, 0xED };
IPAddress ip(192, 168, 125, 3);
EthernetServer server(80);

// Track pin states
bool pinStates[3] = { false, false, false }; // DO0, DO1, DO2

void setup() {
    Serial.begin(9600);
    Ethernet.begin(mac, ip);
    server.begin();

    pinMode(CONTROLLINO_DO0, OUTPUT);
    pinMode(CONTROLLINO_DO1, OUTPUT);
    pinMode(CONTROLLINO_DO2, OUTPUT);

    digitalWrite(CONTROLLINO_DO0, LOW);
    digitalWrite(CONTROLLINO_DO1, LOW);
    digitalWrite(CONTROLLINO_DO2, LOW);

    Serial.print("Server is running at: ");
    Serial.println(ip);
}

void loop() {
    EthernetClient client = server.available();
    if (client) {
        Serial.println("New Client Connected");
        String request = "";
        boolean currentLineIsBlank = true;

        while (client.connected()) {
            if (client.available()) {
                char c = client.read();
                request += c;
                Serial.write(c);

                if (c == '\n' && currentLineIsBlank) {
                    // Parse pin number from URL
                    int pinIndex = -1;
                    if (request.indexOf("GET /s0") != -1) pinIndex = 0;
                    else if (request.indexOf("GET /s1") != -1) pinIndex = 1;
                    else if (request.indexOf("GET /s2") != -1) pinIndex = 2;

                    if (pinIndex != -1) {
                        if (request.indexOf("state=on") != -1) {
                            pinStates[pinIndex] = true;
                        } else if (request.indexOf("state=off") != -1) {
                            pinStates[pinIndex] = false;
                        }
                        // Set the actual pin
                        int pin;
                        if (pinIndex == 0) pin = CONTROLLINO_DO0;
                        else if (pinIndex == 1) pin = CONTROLLINO_DO1;
                        else pin = CONTROLLINO_DO2;
                        digitalWrite(pin, pinStates[pinIndex] ? HIGH : LOW);
                    }

                    // Send response
                    client.println("HTTP/1.1 200 OK");
                    client.println("Content-Type: application/json");
                    client.println("Connection: close");
                    client.println();

                    client.print("{");
                    for (int i = 0; i < 3; i++) {
                        client.print("\"s");
                        client.print(i);
                        client.print("\": ");
                        client.print(pinStates[i] ? "true" : "false");
                        if (i < 2) client.print(", ");
                    }
                    client.println("}");

                    break;
                }

                if (c == '\n') currentLineIsBlank = true;
                else if (c != '\r') currentLineIsBlank = false;
            }
        }

        delay(100);
        client.flush();
        client.stop();
        Serial.println("Client Disconnected");
    }
}
