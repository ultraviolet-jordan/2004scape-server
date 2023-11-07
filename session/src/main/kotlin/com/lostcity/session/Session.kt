package com.lostcity.session

import java.nio.ByteBuffer
import java.security.interfaces.RSAPrivateCrtKey
import kotlin.reflect.KClass

class Session(
    val seed: Long,
    val pem: RSAPrivateCrtKey,
    val codecs: SessionCodecs
) {
    suspend inline fun <reified C : CodecEncoderType, reified T : CodecEncoder<C>> codec(type: KClass<T>, message: C): ByteBuffer? {
        return codecs.encoders.find(type.java::isInstance)?.encode(this, message)
    }

    suspend inline fun <reified R : CodecDecoderType, reified T : CodecDecoder<R>> codec(type: KClass<T>, buffer: ByteBuffer): R? {
        return codecs.decoders.find(type.java::isInstance)?.decode(this, buffer) as R?
    }
}