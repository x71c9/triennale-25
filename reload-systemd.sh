#!/bin/bash

sudo systemctl stop director.service
sudo systemctl daemon-reload
sudo systemctl reenable director.service
sudo systemctl start director.service
