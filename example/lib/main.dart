import 'package:flutter/material.dart';
import 'package:flutter_wry/flutter_wry.dart';

Future<void> main() async {
  await FlutterWry.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  void onPressed() {
    FlutterWry.createWebview(
      WryWebViewConfig(initialUrl: 'https://flutter.dev'),
    );
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('Flutter Wry Example')),
        body: Center(
          child: Column(
            children: [
              Text('Hello, Flutter with Rust and Wry!'),
              IconButton(
                onPressed: onPressed,
                icon: const Text("Create Webview"),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
