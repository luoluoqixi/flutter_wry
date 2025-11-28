import 'dart:io';

import 'rust/api/wry_webview.dart' as wry_webview;
import 'rust/frb_generated.dart' show RustLib;
import 'rust/wry_webview/wry_webview_config.dart';
import 'package:window_manager/window_manager.dart';

class FlutterWry {
  FlutterWry._();

  static final FlutterWry instance = FlutterWry._();
  bool _initialized = false;

  Future<void> init() async {
    if (_initialized) return;
    _initialized = true;
    await RustLib.init();
  }

  Future<void> createWebview(WryWebViewConfig config) async {
    if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
      await windowManager.ensureInitialized();
      int handle = await windowManager.getId();
      // print("handle: $handle");
      wry_webview.createWebview(config: config, handle: handle);
    }
  }
}

final flutterWry = FlutterWry.instance;
