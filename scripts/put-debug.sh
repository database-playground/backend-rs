#!/usr/bin/env bash

plist='<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "https://www.apple.com/DTDs/PropertyList-1.0.dtd"><plist version="1.0"><dict><key>com.apple.security.get-task-allow</key><true/></dict></plist>'

echo "$plist" > /tmp/debug.plist
codesign -f -s - --entitlements /tmp/debug.plist target/debug/backend
