import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:dio/dio.dart';

import 'history.dart';
import 'printer.dart';

class ApiService {
  final Dio _dio;

  ApiService()
    : _dio = Dio(
        BaseOptions(
          baseUrl: _baseUrl,
          connectTimeout: const Duration(seconds: 5),
          receiveTimeout: const Duration(seconds: 10),
        ),
      );

  static const _configuredBaseUrl = String.fromEnvironment('API_BASE_URL');
  static String get _baseUrl {
    if (_configuredBaseUrl.isNotEmpty) return _configuredBaseUrl;
    if (kIsWeb) return '${Uri.base.origin}/api';
    return 'http://127.0.0.1:3000/api';
  }

  Future<List<HistoryItem>> fetchHistory() async {
    final response = await _dio.get<List<dynamic>>('/print-records');
    return (response.data ?? const [])
        .whereType<Map>()
        .map((item) => HistoryItem.fromJson(Map<String, dynamic>.from(item)))
        .toList();
  }

  Future<List<PrinterInfo>> fetchPrinters() async {
    final response = await _dio.get<List<dynamic>>('/print-status');
    return (response.data ?? const [])
        .whereType<Map>()
        .map((item) => PrinterInfo.fromJson(Map<String, dynamic>.from(item)))
        .toList();
  }

  Future uploadFile(File f) async {
    final res = await _dio.post<dynamic>('/upload', data: f);
    return (res.data ?? const [])
        .whereType<Map>()
        .map((item) => PrinterInfo.fromJson(Map<String, dynamic>.from(item)))
        .toList();
  }
}
