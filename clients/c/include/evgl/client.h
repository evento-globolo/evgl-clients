#ifndef EVGL_CLIENT_H
#define EVGL_CLIENT_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct evgl_client {
  const char *base_url;
  const char *token;
} evgl_client;

int evgl_client_endpoint(const evgl_client *client, const char *path,
                         char *output, size_t output_size);

#ifdef __cplusplus
}
#endif

#endif
