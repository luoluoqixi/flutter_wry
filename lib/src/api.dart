import 'rust/api/wry_webview.dart' as wry_webview;
import 'rust/frb_generated.dart' show RustLib;
import 'rust/wry_webview/wry_webview_config.dart';

class FlutterWry {
  static bool _initialized = false;

  static Future<void> init() async {
    if (_initialized) return;
    _initialized = true;
    await RustLib.init();
  }

  static void createWebview(WryWebViewConfig config) {
    wry_webview.createWebview(config: config);
  }
}
