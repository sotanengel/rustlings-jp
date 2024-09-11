import re

# ファイルパス
input_file = "original_Cargo.toml.text"
output_file = "paste_data.text"

# input_fileからデータをパースする関数
def parse_a_text(text):
    # bin配列内のnameとpathをパース
    pattern = r'{\s*name\s*=\s*"([^"]+)",\s*path\s*=\s*"([^"]+)"\s*}'
    matches = re.findall(pattern, text)
    
    exercises = []
    for name, path in matches:
        # pathからディレクトリ名を抽出
        dir_name = path.split("/")[1]
        
        if path.split("/")[0] == "solutions":
            continue

        exercises.append({"name": name, "dir": dir_name})
    
    return exercises

# b.textに出力する関数
def write_b_text(exercises, output_file):
    with open(output_file, "w") as f:
        for exercise in exercises:
            f.write('[[exercises]]\n')
            f.write('# Exercise name which is the exercise file name without the `.rs` extension.\n')
            f.write(f'name = "{exercise["name"]}"\n')
            f.write(f'dir = "{exercise["dir"]}"\n')
            f.write(f'test = false\n')
            f.write('hint = """???"""\n\n')

# a.textを読み込み、パースしてb.textに書き込む
with open(input_file, "r") as f:
    text = f.read()

exercises = parse_a_text(text)
write_b_text(exercises, output_file)

print("output_fileファイルが生成されました。")
