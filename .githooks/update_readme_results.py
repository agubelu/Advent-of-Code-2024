#! /usr/bin/env python
# Automatically adds links to completed days and updates total counts

total_stars = 0
total_runtime = 0.0

def main():
    out_lines = []

    with open('README.md', 'r', encoding='utf-8') as f:
        for line in f.read().splitlines():  # Remove newlines
            out_lines.append(process_line(line))

    with open('README.md', 'w', encoding='utf-8') as f:
        f.writelines(line + '\n' for line in out_lines)

def process_line(line: str) -> str:
    global total_stars
    global total_runtime

    if not line.startswith('|'):
        return line
    spl = [x.strip() for x in line.split('|')]

    if 'Total' in spl[1]:
        # Update totals line
        return f'| **Total** | {total_stars}⭐ | {total_runtime:.4f} ms |'
    elif '⭐' in spl[2]:
        # Day results, add link if missing and update totals
        total_stars += spl[2].count('⭐')
        total_runtime += float(spl[3].split()[0])
        if spl[1].startswith('Day'):
            spl[1] = make_link(spl[1])
        return ' | '.join(spl).strip()

    return line

def make_link(day_str: str) -> str:
    day = int(day_str.split()[1])
    return f'[{day_str}](https://adventofcode.com/2024/day/{day})'

if __name__ == '__main__':
    main()
