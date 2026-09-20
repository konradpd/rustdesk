import 'dart:ffi';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

/// `MB_ICONEXCLAMATION`, plays the system "Exclamation" sound.
const int _kMbIconExclamation = 0x00000030;

typedef _MessageBeepNative = Int32 Function(Uint32 uType);
typedef _MessageBeepDart = int Function(int uType);

_MessageBeepDart? _messageBeep;
bool _messageBeepResolved = false;

_MessageBeepDart? _lookupMessageBeep() {
  if (!_messageBeepResolved) {
    _messageBeepResolved = true;
    try {
      _messageBeep = DynamicLibrary.open('user32.dll')
          .lookupFunction<_MessageBeepNative, _MessageBeepDart>('MessageBeep');
    } catch (e) {
      debugPrint('Failed to resolve MessageBeep: $e');
    }
  }
  return _messageBeep;
}

/// Play the platform alert sound, used to draw attention to something that
/// happened in a window or tab the user is not currently looking at.
///
/// The Windows embedder does not implement `SystemSound.play`, so `MessageBeep`
/// is called directly there.
void playAlertSound_() {
  if (Platform.isWindows) {
    final messageBeep = _lookupMessageBeep();
    if (messageBeep != null) {
      messageBeep(_kMbIconExclamation);
      return;
    }
  }
  SystemSound.play(SystemSoundType.alert);
}
