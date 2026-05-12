import torch
print("CUDA mavjudmi?:", torch.cuda.is_available())
print("GPU nomi:", torch.cuda.get_device_name(0))


# [
#     [  <-- 1-gapning boshlanishi
#         [-0.012, 0.002, 0.012, ..., 512-son],  # 12-ID (masalan: "Men")
#         [-0.013, 0.042, 0.052, ..., 512-son],  # 15-ID (masalan: "maktabga")
#         [ 0.001, -0.01, 0.033, ..., 512-son],  # 64-ID (masalan: "boraman")
#         [ 0.000, 0.000, 0.000, ..., 512-son],  # 0-ID  (Padding - bo'sh joy)
#         ... (jami 2048 ta qator bo'lguncha)
#     ]  <-- 1-gapning yopilishi
#     [  <-- 2-gapning boshlanishi
#         [-0.012, 0.002, 0.012, ..., 512-son],  # 12-ID (masalan: "Sen")
#         [-0.013, 0.042, 0.052, ..., 512-son],  # 15-ID (masalan: "Uyga")
#         [ 0.001, -0.01, 0.033, ..., 512-son],  # 64-ID (masalan: "borasan")
#         [ 0.000, 0.000, 0.000, ..., 512-son],  # 0-ID  (Padding - bo'sh joy)
#         ... (jami 2048 ta qator bo'lguncha)
#     ]  <-- 2-gapning yopilishi
#     ...
    
#     64 ta gapgacha
# ]
# 64, 2048, 512 bu eng muhum nuqta
# 64 bu gapning chegarasi
# 2048 [ ichiga qancah uzunlikdagi gaplarni sig'dirish sozlamasi] 128 256 512 1024 2048 o'lchiv birliklar bilan qilinadi
# 512 weights beriladi har bitta sozga