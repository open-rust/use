#include <stdio.h>
#include <stdint.h>
#include <math.h>
#include <Windows.h>

#include <thread>

#define M_PI       3.14159265358979323846   // pi

// 生成带淡入淡出的正弦波 WAV 数据到内存缓冲区
// frequency: 频率（Hz），推荐警告音 800Hz 到 1000 Hz
// duration_seconds: 生成的样本时长（秒），1~2 秒足够循环
// fade_seconds: 淡入淡出时长（秒）
// 返回: 动态分配的缓冲区指针（调用者需 delete[] 释放），大小通过 out_size 返回
extern "C" static
BYTE* GenerateSineWaveWav(double frequency, double duration_seconds, double fade_seconds, DWORD* out_size) {
    const DWORD sampleRate = 44100;     // 采样率（CD 质量）
    const WORD channels = 1;            // 单声道
    const WORD bitsPerSample = 16;      // 16 位

    DWORD numSamples = static_cast<DWORD>(sampleRate * duration_seconds);
    DWORD dataSize = numSamples * channels * (bitsPerSample / 8);
    DWORD fileSize = 36 + dataSize;     // WAV 标准头大小 44 字节 - 8

    BYTE* wav = new BYTE[44 + dataSize];

    // RIFF 头
    memcpy(wav + 0,  "RIFF", 4);
    *(DWORD*)(wav + 4) = fileSize;
    memcpy(wav + 8,  "WAVE", 4);

    // fmt 子块
    memcpy(wav + 12, "fmt ", 4);
    *(DWORD*)(wav + 16) = 16;                   // 子块大小
    *(WORD*)(wav + 20) = 1;                     // PCM 格式
    *(WORD*)(wav + 22) = channels;
    *(DWORD*)(wav + 24) = sampleRate;
    *(DWORD*)(wav + 28) = sampleRate * channels * (bitsPerSample / 8);  // 字节率
    *(WORD*)(wav + 32) = channels * (bitsPerSample / 8);               // 对齐
    *(WORD*)(wav + 34) = bitsPerSample;

    // data 子块
    memcpy(wav + 36, "data", 4);
    *(DWORD*)(wav + 40) = dataSize;

    // 生成正弦波样本（16 位有符号）
    short* samples = (short*)(wav + 44);
    DWORD fadeSamples = static_cast<DWORD>(sampleRate * fade_seconds);

    for (DWORD i = 0; i < numSamples; ++i)
    {
        double t = i / (double)sampleRate;
        double sine = sin(2.0 * M_PI * frequency * t);

        // 淡入 + 持续 + 淡出
        double envelope = 1.0;
        if (i < fadeSamples) // 淡入（线性或余弦更柔和，这里用余弦）
            envelope = (1.0 - cos(M_PI * i / fadeSamples)) * 0.5;
        else if (i >= numSamples - fadeSamples) // 淡出
            envelope = (1.0 + cos(M_PI * (i - (numSamples - fadeSamples)) / fadeSamples)) * 0.5;

        samples[i] = static_cast<short>(sine * envelope * 32767 * 0.8);
    }

    *out_size = 44 + dataSize;
    return wav;
}

// 生成逐渐衰减的正弦波 WAV 数据到内存缓冲区
// frequency: 频率（Hz），推荐警告音 800Hz 到 1000 Hz
// duration_seconds: 生成的样本时长（秒），1~2 秒足够循环
// 返回: 动态分配的缓冲区指针（调用者需 delete[] 释放），大小通过 out_size 返回
extern "C" static
BYTE* GenerateDing(double frequency, double duration_seconds, DWORD* out_size) {
    const DWORD sampleRate = 44100;     // 采样率（CD 质量）
    const WORD channels = 2;            // 双声道
    const WORD bitsPerSample = 16;      // 16 位

    DWORD numSamples = static_cast<DWORD>(sampleRate * duration_seconds);
    DWORD dataSize = numSamples * channels * (bitsPerSample / 8);
    DWORD fileSize = 36 + dataSize;     // WAV 标准头大小 44 字节 - 8

    BYTE* wav = new BYTE[44 + dataSize];

    // RIFF 头
    memcpy(wav + 0,  "RIFF", 4);
    *(DWORD*)(wav + 4) = fileSize;
    memcpy(wav + 8,  "WAVE", 4);

    // fmt 子块
    memcpy(wav + 12, "fmt ", 4);
    *(DWORD*)(wav + 16) = 16;                   // 子块大小
    *(WORD*)(wav + 20) = 1;                     // PCM 格式
    *(WORD*)(wav + 22) = channels;
    *(DWORD*)(wav + 24) = sampleRate;
    *(DWORD*)(wav + 28) = sampleRate * channels * (bitsPerSample / 8);  // 字节率
    *(WORD*)(wav + 32) = channels * (bitsPerSample / 8);               // 对齐
    *(WORD*)(wav + 34) = bitsPerSample;

    // data 子块
    memcpy(wav + 36, "data", 4);
    *(DWORD*)(wav + 40) = dataSize;

    // 生成逐渐衰减的正弦波（更自然）
    short* samples = (short*)(wav + 44);
    for (int i = 0; i < numSamples; i++) {
        double t = i / (double)sampleRate;
        double envelope = 1.0 - (i / (double)numSamples);  // 线性衰减
        samples[i] = (short)(10000 * envelope * sin(2.0 * M_PI * frequency * t));
    }

    *out_size = 44 + dataSize;
    return wav;
}

static uint32_t playing = 0;
extern "C"
void beep(uint32_t time) {
    // Beep(440, time);
    // MessageBeep(MB_OK);

    DWORD wavSize = 0;

    // 生成 1000 Hz 正弦波，持续 1 秒（循环后无限）
    // BYTE* wavData = GenerateSineWaveWav(1000.0, 1, 0.2, &wavSize);

    // 生成 1000 Hz 正弦波，持续 1 秒（循环后无限）
    BYTE* wavData = GenerateDing(1000.0, 1, &wavSize);

    // printf("wavSize: %d\n", wavSize);

    // 循环播放警告音（异步 + 循环）
    PlaySoundA((LPCSTR)wavData, NULL, SND_MEMORY | SND_ASYNC | SND_LOOP);
    playing++;

    std::thread cancel([=] {
        Sleep(time);
        if (playing < 2) {
            PlaySoundA((LPCSTR)NULL, NULL, SND_SYNC);
        }
        playing--;
    });
    cancel.detach();
    delete[] wavData;
}