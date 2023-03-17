<div align="center">
  <h1> Context Discord Bot v0.0.1</h1>
  <br>
  <strong><i>Give Context. Receive Answers.</i></strong>
  <br>
  <br>

  <a href="#">
  <img width="336" alt="SCR-20230316-or2" src="https://user-images.githubusercontent.com/63879791/225760095-dafc86a3-4461-4359-afc7-f9261986e067.png">

  </a>

  <br>

  <a href="https://www.python.org/downloads/">
    <img src="https://user-images.githubusercontent.com/63879791/225761679-e827fe4b-0a41-42f3-ab5a-6518ac981355.png" alt="Made with RUST">
  </a>

<br>
  <a href="https://discord.gg/NDshuddJ">
    <img src="https://img.shields.io/discord/515071617815019520.svg?label=Discord&logo=Discord&colorB=7289da&style=for-the-badge" alt="Support">
  </a>
<br>
  <strong><i>A Discord Bot that can answer ANY question given the context.</i></strong>

  <img src = https://user-images.githubusercontent.com/63879791/226038822-85e624da-551e-44fb-9b54-9ba5018c4441.gif>
</div>

## What is Context?

Context is a Discord bot that allows users to ask any question given the specific context and recieve accurate and speedy answers. 

## What Machine Learning Model Does It Use?

The DistilBERT model was proposed in the blog post <a href ="https://medium.com/huggingface/distilbert-8cf3380435b5">Smaller, faster, cheaper, lighter: Introducing DistilBERT, adistilled version of BERT, and the paper <a href="https://arxiv.org/abs/1910.01108">DistilBERT, adistilled version of BERT: smaller, faster, cheaper and lighter</a>. DistilBERT is a small, fast, cheap and light Transformer model trained by distilling BERT base </a>. It has 40% less parameters than bert-base-uncased, runs 60% faster while preserving over 95% of BERT's performances as measured on the GLUE language understanding benchmark. <a href= "https://huggingface.co/distilbert-base-cased-distilled-squad#model-details">Hugging Face</i>

## How does it work?

When a member uses a slash command to initiate the bot, they need to input the context and a question (Be aware, if the Discord Bot has been idle, you might need to wait 20 seconds to initiate the model). Context, the Discord Bot, will receive these two inputs and then calculate an answer. 

## Credits and Acknowledgements

Thank you to every Rust programming tutorial and documentation that has ever been posted. The Rust community is by far one of the best, and I am very happy to be apart of it now.

Thank you to <a href="https://shuttle.rs/"> Shuttle.rs </a> for providing a hosting platform for my Discord Bot and an amazing tutorial on <a href="https://www.shuttle.rs/blog/2022/09/14/serentity-discord-bot">How to Make A Discord Bot in Rust. </a>

And Lastly, Thank you to Michael Francis for taking the precious time to explain what a compiled language was and letting me know more about Rust. You are the best!

