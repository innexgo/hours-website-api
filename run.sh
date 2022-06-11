#!/bin/bash
./target/debug/hours-contact-service \
  --port=8080 \
  --mail-service-url=http://localhost:8078
