class EvglClient {
  EvglClient(String baseUrl, {this.bearerToken})
    : baseUrl = Uri.parse(
        baseUrl.endsWith('/')
            ? baseUrl.substring(0, baseUrl.length - 1)
            : baseUrl,
      );

  final Uri baseUrl;
  final String? bearerToken;

  Uri endpoint(String path) =>
      baseUrl.resolve(path.startsWith('/') ? path.substring(1) : path);
  Uri get health => endpoint('/healthz');
  Uri get config => endpoint('/api/config');
  Uri get providers => endpoint('/v1/providers');
  Uri get connections => endpoint('/v1/connections');
  Uri get events => endpoint('/v1/events');
  Uri startOAuth(String provider) =>
      endpoint('/v1/oauth/${Uri.encodeComponent(provider)}/start');
  Uri event(String eventId) =>
      endpoint('/v1/events/${Uri.encodeComponent(eventId)}');
  Uri crossPost(String eventId) =>
      endpoint('/v1/events/${Uri.encodeComponent(eventId)}/cross-post');
  Uri job(String jobId) => endpoint('/v1/jobs/${Uri.encodeComponent(jobId)}');
  Uri jobWebSocket(String jobId) {
    final uri = endpoint('/v1/jobs/${Uri.encodeComponent(jobId)}/ws');
    return uri.replace(scheme: uri.scheme == 'https' ? 'wss' : 'ws');
  }
}
