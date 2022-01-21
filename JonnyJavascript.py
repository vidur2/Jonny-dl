from youtube_dl import YoutubeDL


def main():
    ytdl = YoutubeDL()
    video = input("Enter video Download")
    ytdl.downlaod(video)