package io.zedpkg.evgl
import java.net.URI
data class EvglClient(val baseUri: URI, val bearerToken: String? = null)
