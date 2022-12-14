use rsa_oaep_pss::{FromPem, RsaError};

const PUBLIC_KEY_PEM: &str = 
"-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEAxuGJbAzGqmuvQxylAj9UWsbMn0DmXvrSVqOtbBOoBxutZnyr2Uf/
eB3DtlepcRWtrKEhASqENtWAXg/HkqtbbYN0O/Iq0BMsy/MOtMNVkuyP6nUhc+UO
fecE8oBRbuHjpVpwCtChRCVY8YUq/d2n5mTVz1ADPz1PUEa3ESgJcLxU9riQh+//
p9ikoxtqsUQiiqlzMhTNxkuUlYoITBiD2zPkXKlmTuH9fYW0QGt52L2cht6VXixW
FrwsiwoD+J/WhRACNWuw7szyQiItetmfacllcamEmC0MSr1ly4WhEyC2ESh0LB7v
E2mv+fJ+0eq4JucUK6aqEm6z5C2o7Mk4JwIDAQAB
-----END RSA PUBLIC KEY-----
";

const PRIVATE_KEY_PEM: &str = 
"-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAxuGJbAzGqmuvQxylAj9UWsbMn0DmXvrSVqOtbBOoBxutZnyr
2Uf/eB3DtlepcRWtrKEhASqENtWAXg/HkqtbbYN0O/Iq0BMsy/MOtMNVkuyP6nUh
c+UOfecE8oBRbuHjpVpwCtChRCVY8YUq/d2n5mTVz1ADPz1PUEa3ESgJcLxU9riQ
h+//p9ikoxtqsUQiiqlzMhTNxkuUlYoITBiD2zPkXKlmTuH9fYW0QGt52L2cht6V
XixWFrwsiwoD+J/WhRACNWuw7szyQiItetmfacllcamEmC0MSr1ly4WhEyC2ESh0
LB7vE2mv+fJ+0eq4JucUK6aqEm6z5C2o7Mk4JwIDAQABAoIBAQCKJt+ELDfQ6G7d
yschEYbybAh2U+nzbn6wapCsc9cT4PkKKW5iyC3BbcOnJavxfyKeuWso2TTwtkyu
in2S9+/R2i/i+G0fD0LeBeBgVxRZtHSgZG8xh5sSgGkiX1qyEL5G/dS43oY3xU3M
sDnWEaRxWfhgQ2+N/XPcCtNeWpZ8NU0X905CiHUWMmN4XLaPICG3qfltb7zlM2g1
8jniE55jR1v+BoqY4UADP61PpIGtRh93154quOyvPJJnrHNaTS0KNIjv1uK+6U9i
+6WQX+fJUJ+3FzypoAs2XQ1uUtoX3ksNs9zFt1bW7LFD8mLj+GbYU+ld4ZzCOtcf
E+1dPhVxAoGBAPC25by356zwYD9xzMsoHZeP5P0Q7iYhSATf39eBly+/s8KEkVLH
S6lKcnIBu0AOcXqkGWRMuxNt4ZxT+YBy3ZW28/wpyX3d1gfIsGvyr4IhtT3KuZrh
spRDFLgLSRrvUC9oONwI80PwQjokYeVo2rS4o14acnPh5nICcqToyptpAoGBANOC
ligRM8yB+SuckIot7m9E6BK8UxgD1+CcHaM3sI5PGeeuKfGOmD3OBszP4fHtByW6
ClR0G6kUx66vsLzjEwBntjxaXy76sizbCUiWEAheCQx5838RaJZ1+ZlTNeoOVNkT
JLKqL1wg8mL8rT7ERURWEDFAhLSUU0dGYxT4BpUPAoGAK3N4aOZxhYAmf41VQF7P
aKpq1VuXAX8EdwzKZSZZWh8qnbQ8X0rmV3q+yeGSIW/5//9bLgXDxzAQh52I7izN
3taVmDEa1AOVPPZ3EiVOlRjslGjhtBmFmMm7FbXDzn7gZI5VpTJgZyUZb1Fd8oFI
6VJi+vufKVfftzjF5Qq4d/ECgYBy3Gm63IyqtLKh0mZUA9mGJ394HTlqwzuKyHz3
1BhXT/GbdWNlwLBAcBAVTZdAEB5fiIZvkpYq3hQKc3KhQRdiku/OGq2yD4aPAWwr
NQMD6D01v0xSZxvLCcpAT4Xp01qUf+bf97e8ak/3HYzflyqej7Y/aPAoOJx5Cmpo
SsEYVQKBgAqbQPIEybcKm9JvoEcNVXPaVOReMavPbGmrGETj1bqxKK8KghA+jEwh
4afMlG7fM4xQD4y2HCWDGCBrLlGgdjMO0yMUEtPUhkxQqgbD0z5rUiKAJlRsN7m5
3F7MBMeNFX9t3Ok25TIcFQwWUGW5kORvZAdoQ4/OZz8wzDeTNtRr
-----END RSA PRIVATE KEY-----
";

fn main () -> Result<(), RsaError> {

    let _public_key = rsa_oaep_pss::RsaPublicKey::from_pem(&PUBLIC_KEY_PEM)?;
    let _private_key = rsa_oaep_pss::RsaPrivateKey::from_pem(&PRIVATE_KEY_PEM)?;

    Ok(())
}