import 'package:flutter/material.dart';
import 'package:flutter_wry/flutter_wry.dart';

Future<void> main() async {
  await flutterWry.init();
  runApp(MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  void initState() {
    super.initState();
  }

  void onPressed() {
    flutterWry.createWebview(
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
