import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiClient {
  ApiClient(this.baseUrl, {this.token, http.Client? client})
      : client = client ?? http.Client();

  final Uri baseUrl;
  final String? token;
  final http.Client client;

  Future<Map<String, dynamic>> health() async {
    final response = await client.get(
      baseUrl.resolve('/healthz'),
      headers: token == null ? {} : {'authorization': 'Bearer $token'},
    );
    if (response.statusCode >= 300) {
      throw StateError('API request failed: ${response.statusCode}');
    }
    return jsonDecode(response.body) as Map<String, dynamic>;
  }
}
