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
      this.chatHandlers.forEach(handler => {
        try {
          handler(mockMessage);
        } catch (error) {
          console.error('Failed to handle chat message:', error);
        }
      });
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
    try {
      console.log(`Message received from ${message.username}: ${message.message}`);
    } catch (error) {
      console.error(`Error handling chat message: ${error}`);
    }
  }

  createPoll(question: string, options: string[]): Poll {
    try {
      if (!question || options.length === 0) {
        throw new Error('Poll must have a question and at least one option.');
      }
  
      const newPoll: Poll = {
        question,
        options,
        voteTally: new Map(),
      };
  
      this.activePolls.push(newPoll);
      console.log(`Poll created with question: ${question}`);
      return newPoll;
    } catch (error) {
      console.error('Failed to create poll:', error);
      throw error;  // Re-throwing to signal that an unrecoverable error occurred.
    }
  }

  castVote(pollIndex: number, option: string): void {
    try {
      if (pollIndex < 0 || pollIndex >= this.activePolls.length) {
        console.error(`Poll at index ${pollIndex} does not exist.`);
        return;
      }
  
      const selectedPoll = this.activePolls[pollIndex];
      if (!selectedPoll.options.includes(option)) {
        console.error(`Option "${option}" is not valid for the poll.`);
        return;
      }
  
      const votesForOption = selectedPoll.voteTally.get(option) || 0;
      selectedPoll.voteTally.set(option, votesForOption + 1);
      console.log(`Vote for "${option}" recorded.`);
    } catch (error) {
      console.error('Failed to cast vote:', error);
    }
  }

  submitQuestion(question: string): void {
    try {
      if (!question.trim()) {
        console.error('Question cannot be empty.');
        return;
      }
  
      this.faqSection.push({
        question,
        answer: null,
      });
      console.log(`Question submitted: ${question}`);
    } catch (error) {
      console.error('Failed to submit question:', error);
    }
  }

  answerFAQ(questionIndex: number, answer: string): void {
    try {
      if (questionIndex < 0 || questionIndex >= this.faqSection.length) {
        console.error(`FAQ item at index ${questionIndex} does not exist.`);
        return;
      }
  
      const faqItem = this.faqSection[questionIndex];
      if (!faqItem) {
        console.error(`FAQ Item with Index ${questionIndex} not found.`);
        return;
      }
  
      if (faqItem.answer !== null) {
        console.error(`FAQ Item with Index ${questionIndex} has already been answered.`);
        return;
   }
  
      faqItem.answer = answer;
      console.log(`Answer provided: ${answer}`);
    } catch (error) {
      console.error('Failed to answer FAQ:', error);
    }
  }
}

const streamController = new InteractionController();
streamController.createPoll("Who is your favorite character?", ["Character A", "Character B", "Character C"]);
streamController.submitQuestion("How to implement feature X in language Y?");