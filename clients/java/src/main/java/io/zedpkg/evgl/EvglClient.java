package io.zedpkg.evgl;
import java.net.URI;
public record EvglClient(URI baseUri, String bearerToken) {}
