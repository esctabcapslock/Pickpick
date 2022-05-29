# %%
from PIL import Image
img_file = r'icon_test4.png'
img=Image.open(img_file).convert('RGBA')
pixels = [*img.getdata()]

# %%
f = open("outout.txt",'w')
for i in pixels:
    f.write(f"{i[0]},{i[1]},{i[2]},{i[3]},\n")
      