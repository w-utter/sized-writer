An indexed writer for buffers, primarily used when serializing data of an unknown size (such as JSON)

**Motivation**
 - &mut [u8] does not allow consecutive writes without overwriting existing data
 - std::io::BufWriter<_> uses heap allocation in its implementation, which is unnecessary for a lot of applications
 - No existing implementation keeps track of whether the fixed buf has been filled up and overwritten
