import dotenv from 'dotenv';
dotenv.config();

interface ChatMessage {
  username: string;
  message: string;
}

interface Poll {
  question: string;
  options: string[];
  responses: Map<string, number>;
}

interface QnA {
  question: string;
  answer: string | null;
}

class ChatSystem {
  private listeners: ((message: ChatMessage) => void)[] = [];

  constructor() {
    setInterval(() => {
      const mockMessage: ChatMessage = {
        username: 'Viewer_' + Math.floor(Math.random() * 100),
        message: 'This is a test message!',
      };
      this.listeners.forEach(listener => listener(mockMessage));
    }, 1000);
  }

  onMessageReceived(listener: (message: ChatMessage) => void): void {
    this.listeners.push(listener);
  }
}

class LiveStreamInteractionManager {
  private chatSystem: ChatSystem;
  private polls: Poll[] = [];
  private qAndASession: QnA[] = [];

  constructor() {
    this.chatSystem = new ChatSystem();
    this.chatSystem.onMessageReceived(this.handleChatMessage.bind(this));
  }

  private handleChatMessage(message: ChatMessage): void {
    console.log(`Message received from ${message.username}: ${message.message}`);
  }

  createPoll(question: string, options: string[]): Poll {
    const poll: Poll = {
      question,
      options,
      responses: new Map(),
    };
    this.polls.push(poll);
    return poll;
  }

  voteInPoll(pollId: number, option: string): void {
    const poll = this.polls[pollId];
    if (poll) {
      const currentVotes = poll.responses.get(option) || 0;
      poll.responses.set(option, currentVotes + 1);
      console.log(`Vote recorded for option: ${option}`);
    } else {
      console.log(`Poll with ID ${pollId} not found.`);
    }
  }

  askQuestion(question: string): void {
    this.qAndASession.push({
      question,
      answer: null,
    });
    console.log(`Question asked: ${question}`);
  }

  answerQuestion(questionId: number, answer: string): void {
    const questionObj = this.qAndASession[questionId];
    if (questionObj && questionObj.answer === null) {
      questionObj.answer = answer;
      console.log(`Answered: ${answer}`);
    } else {
      console.log(`Question with ID ${questionId} was already answered or not found.`);
    }
  }
}

const manager = new LiveStreamInteractionManager();
manager.createPoll("Who is your favorite character?", ["Character A", "Character B", "Character C"]);
manager.askQuestion("How to implement feature X in language Y?");