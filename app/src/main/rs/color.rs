

#pragma version(1)
#pragma rs java_package_name(com.codecraft.renderscriptplayground)
#pragma rs_fp_relaxed

float mixFactor = 0.5f;
int32_t red;
int32_t green;
int32_t blue;

uchar4 __attribute__((kernel)) color(uchar4 in, uint32_t x, uint32_t y) {
  uchar4 out = in;

  out.r =  mixFactor * (red - out.r) + out.r;
  out.g = mixFactor * (green - out.g) + out.g;
  out.b = mixFactor * (blue - out.b) + out.b;

  return out;
}

uchar4 __attribute__((kernel)) lumaColor(uchar4 in, uint32_t x, uint32_t y) {
  uchar4 out = in;

  float mix = clamp(mixFactor * sqrt((299 * in.r + 587 * in.g + 114 * in.b) / (255000.f)), 0.f, 1.f);
  out.r =  mix * (red - out.r) + out.r;
  out.g = mix * (green - out.g) + out.g;
  out.b = mix * (blue - out.b) + out.b;

  return out;
}

