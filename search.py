import pyexcel
from thefuzz import process

records = pyexcel.get_records(file_name="JAVA程序设计.xlsx")
contents = [row['content'] for row in records]

def getAnswer(content: str):
    return records[contents.index(process.extractOne(content, contents)[0])]

print(getAnswer("两个"))