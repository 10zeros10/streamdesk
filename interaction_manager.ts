import dotenv from 'dotenv';
dotenv.config();

interface ChatMessage {
  username: string;
  message: string;
}

interface Poll {
  question: string;
  options: string[];
  voteTally: Map<string, number>;
}

interface FAQItem {
  question: string;
  answer: string | null;
}

class ChatMessageListener {
  private chatHandlers: ((message: ChatMessage) => void)[] = [];

  constructor() {
    setInterval(() => {
      const mockMessage: ChatMessage = {
        username: 'Viewer_' + Math.floor(Math.random() * 100),
        message: 'This is a test message!',
      };
      this.chatHandlers.forEach(handler => handler(mockMessage));
    }, 1000);
  }

  onChatMessage(handler: (message: ChatMessage) => void): void {
    this.chatHandlers.push(handler);
  }
}

class InteractionController {
  private chatListener: ChatMessageListener;
  private activePolls: Poll[] = [];
  private faqSection: FAQItem[] = [];

  constructor() {
    this.chatListener = new ChatMessageListener();
    this.chatListener.onChatMessage(this.handleChatMessage.bind(this));
  }

  private handleChatMessage(message: ChatMessage): void {
    console.log(`Message received from ${message.username}: ${message.message}`);
  }

  createPoll(question: string, options: string[]): Poll {
    const newPoll: Poll = {
      question,
      options,
      voteTally: new Map(),
    };
    this.activePolls.push(newPoll);
    return newPoll;
  }

  castVote(pollIndex: number, option: string): void {
    const selectedPoll = this.activePolls[pollIndex];
    if (selectedPoll) {
      const votesForOption = selectedPoll.voteTally.get(option) || 0;
      selectedPoll.voteTally.set(option, votesForOption + 1);
      console.log(`Vote for "${option}" recorded.`);
    } else {
      console.log(`Poll with Index ${pollIndex} not found.`);
    }
  }

  submitQuestion(question: string): void {
    this.faqSection.push({
      question,
      answer: null,
    });
    console.log(`Question submitted: ${question}`);
  }

  answerFAQ(questionIndex: number, answer: string): void {
    const faqItem = this.faqSection[questionIndex];
    if (faqItem && faqItem.answer === null) {
      faqItem.answer = answer;
      console.log(`Answer provided: ${answer}`);
    } else {
      console.log(`FAQ Item with Index ${questionIndex} has been previously answered or not found.`);
    }
  }
}

const streamController = new InteractionController();
streamController.createPoll("Who is your favorite character?", ["Character A", "Character B", "Character C"]);
streamController.submitQuestion("How to implement feature X in language Y?");