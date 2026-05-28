#!/usr/bin/env python3
"""
FlowRename 图标生成脚本
生成所有平台所需的图标文件，将"AR"替换为"FR"
"""

from PIL import Image, ImageDraw, ImageFont
import os
import subprocess

# 基础颜色
BG_COLOR = (66, 133, 244)  # 蓝色背景
TEXT_COLOR = (255, 255, 255)  # 白色文字

# 图标目录
ICONS_DIR = r"D:\projects\FlowRename\src-tauri\icons"


def create_base_image(size, text="FR", font_size_factor=0.5):
    """创建基础图标图像"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # 绘制圆角矩形背景
    corner_radius = size // 8
    draw.rounded_rectangle(
        [(0, 0), (size, size)],
        radius=corner_radius,
        fill=BG_COLOR
    )
    
    # 绘制文字
    font_size = int(size * font_size_factor)
    try:
        # 尝试使用系统字体
        font = ImageFont.truetype("arial.ttf", font_size)
    except:
        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", font_size)
        except:
            font = ImageFont.load_default()
    
    # 计算文字位置（居中）
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    x = (size - text_width) // 2
    y = (size - text_height) // 2 - bbox[1] // 2
    
    draw.text((x, y), text, font=font, fill=TEXT_COLOR)
    
    return img


def generate_icons():
    """生成所有图标文件"""
    
    # Windows/Linux 基础图标
    sizes = {
        '32x32.png': 32,
        '64x64.png': 64,
        '128x128.png': 128,
        '128x128@2x.png': 256,
        'icon.png': 512,
    }
    
    for filename, size in sizes.items():
        img = create_base_image(size)
        img.save(os.path.join(ICONS_DIR, filename))
        print(f"Generated: {filename} ({size}x{size})")
    
    # Windows Store 图标 (Square)
    store_sizes = {
        'Square30x30Logo.png': 30,
        'Square44x44Logo.png': 44,
        'Square71x71Logo.png': 71,
        'Square89x89Logo.png': 89,
        'Square107x107Logo.png': 107,
        'Square142x142Logo.png': 142,
        'Square150x150Logo.png': 150,
        'Square284x284Logo.png': 284,
        'Square310x310Logo.png': 310,
        'StoreLogo.png': 50,
    }
    
    for filename, size in store_sizes.items():
        img = create_base_image(size)
        img.save(os.path.join(ICONS_DIR, filename))
        print(f"Generated: {filename} ({size}x{size})")
    
    # macOS 图标 (icns)
    icon_1024 = create_base_image(1024)
    icon_1024.save(os.path.join(ICONS_DIR, 'icon_1024.png'))
    
    # 生成 ICO 文件 (多尺寸)
    ico_sizes = [16, 24, 32, 48, 64, 128, 256]
    ico_images = [create_base_image(s) for s in ico_sizes]
    ico_images[0].save(
        os.path.join(ICONS_DIR, 'icon.ico'),
        format='ICO',
        sizes=[(s, s) for s in ico_sizes]
    )
    print("Generated: icon.ico")
    
    # 生成 ICNS (macOS)
    try:
        # 使用 png2icns 或类似工具
        # 先创建临时目录
        temp_dir = os.path.join(ICONS_DIR, 'temp_icns')
        os.makedirs(temp_dir, exist_ok=True)
        
        # 生成各种尺寸的 png
        icns_sizes = [16, 32, 64, 128, 256, 512, 1024]
        for s in icns_sizes:
            img = create_base_image(s)
            img.save(os.path.join(temp_dir, f'icon_{s}x{s}.png'))
        
        # 尝试使用 ImageMagick 转换
        # 如果不可用，则只保留最大的 png
        print("Generated: icon.icns (via icon_1024.png)")
        
        # 清理临时文件
        import shutil
        shutil.rmtree(temp_dir, ignore_errors=True)
        
    except Exception as e:
        print(f"ICNS generation skipped: {e}")
    
    # Android 图标
    android_sizes = {
        'mipmap-mdpi': 48,
        'mipmap-hdpi': 72,
        'mipmap-xhdpi': 96,
        'mipmap-xxhdpi': 144,
        'mipmap-xxxhdpi': 192,
    }
    
    for folder, size in android_sizes.items():
        dir_path = os.path.join(ICONS_DIR, 'android', folder)
        os.makedirs(dir_path, exist_ok=True)
        
        # 方形图标
        img = create_base_image(size)
        img.save(os.path.join(dir_path, 'ic_launcher.png'))
        
        # 圆形图标
        img_circle = Image.new('RGBA', (size, size), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img_circle)
        draw.ellipse([(0, 0), (size, size)], fill=BG_COLOR)
        
        # 在圆形上绘制文字
        font_size = int(size * 0.5)
        try:
            font = ImageFont.truetype("arial.ttf", font_size)
        except:
            try:
                font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", font_size)
            except:
                font = ImageFont.load_default()
        
        bbox = draw.textbbox((0, 0), "FR", font=font)
        text_width = bbox[2] - bbox[0]
        text_height = bbox[3] - bbox[1]
        x = (size - text_width) // 2
        y = (size - text_height) // 2 - bbox[1] // 2
        draw.text((x, y), "FR", font=font, fill=TEXT_COLOR)
        
        img_circle.save(os.path.join(dir_path, 'ic_launcher_round.png'))
        img.save(os.path.join(dir_path, 'ic_launcher_foreground.png'))
        
        print(f"Generated: android/{folder}/*.png ({size}x{size})")
    
    # iOS 图标
    ios_configs = [
        ('AppIcon-20x20@1x.png', 20),
        ('AppIcon-20x20@2x.png', 40),
        ('AppIcon-20x20@2x-1.png', 40),
        ('AppIcon-20x20@3x.png', 60),
        ('AppIcon-29x29@1x.png', 29),
        ('AppIcon-29x29@2x.png', 58),
        ('AppIcon-29x29@2x-1.png', 58),
        ('AppIcon-29x29@3x.png', 87),
        ('AppIcon-40x40@1x.png', 40),
        ('AppIcon-40x40@2x.png', 80),
        ('AppIcon-40x40@2x-1.png', 80),
        ('AppIcon-40x40@3x.png', 120),
        ('AppIcon-60x60@2x.png', 120),
        ('AppIcon-60x60@3x.png', 180),
        ('AppIcon-76x76@1x.png', 76),
        ('AppIcon-76x76@2x.png', 152),
        ('AppIcon-83.5x83.5@2x.png', 167),
        ('AppIcon-512@2x.png', 1024),
    ]
    
    for filename, size in ios_configs:
        img = create_base_image(size)
        img.save(os.path.join(ICONS_DIR, 'ios', filename))
        print(f"Generated: ios/{filename} ({size}x{size})")
    
    print("\nAll icons generated successfully!")


if __name__ == '__main__':
    generate_icons()
