import 'package:flutter/services.dart';

/// No `dart:ffi` on web, fall back to the framework's alert sound.
void playAlertSound_() {
  SystemSound.play(SystemSoundType.alert);
}
