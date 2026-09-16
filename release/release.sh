#!/bin/bash

cargo tauri build
makepkg --force --dir release
