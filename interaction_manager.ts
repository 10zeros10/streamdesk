import dotenv from 'dotenv';
dotenv.config();

interface ChatMessage {
  username: string;
  message: string;
}

interface Survey {
  question: string;
  options: string[];
  votes: Map<string, number>; 
}

interface FAQ {
  question: string;
  answer: string | null; 
}

class MessageDispatcher {
  private messageHandlers: ((message: ChatMessage) => void)[] = [];

  constructor() {
    setInterval(() => {
      const testMessage: ChatMessage = { 
        username: 'Viewer_' + Math.floor(Math.random() * 100),
        message: 'This is a test message!',
      };
      this.messageHandlers.forEach(handler => handler(testMessage));
    }, 1000);
  }

  subscribeToMessages(handler: (message: ChatMessage) => void): void { 
    this.messageHandlers.push(handler);
  }
}

class StreamInteractionManager {
  private messageSystem: MessageDispatcher; 
  private activePolls: Survey[] = []; 
  private debateSection: FAQ[] = []; 

  constructor() {
    this.messageSystem = new MessageDispatcher();
    this.messageSystem.subscribeToMessages(this.interpretChatMessage.bind(this)); 
  }

  private interpretChatMessage(message: ChatMessage): void { 
    console.log(`Message received from ${message.username}: ${message.message}`);
  }

  initiatePoll(question: string, options: string[]): Survey { 
    const survey: Survey = { 
      question,
      options,
      votes: new Map(), 
    };
    this.activePolls.push(survey);
    return survey;
  }

  recordVote(pollIndex: number, option: string): void { 
    const survey = this.activePolls[pollIndex]; 
    if (survey) {
      const currentVotes = survey.votes.get(option) || 0;
      survey.votes.set(option, currentVotes + 1);
      console.log(`Vote recorded for option: ${option}`);
    } else {
      console.log(`Survey with Index ${pollIndex} not found.`);
    }
  }

  postQuestion(question: string): void { 
    this.debateSection.push({ 
      question,
      answer: null,
    });
    console.log(`Question posted: ${question}`);
  }

  provideAnswer(questionIndex: number, answer: string): void { 
    const faq = this.debateSection[questionIndex]; 
    if (faq && faq.answer === null) {
      faq.answer = answer;
      console.log(`Answer provided: ${answer}`);
    } else {
      console.log(`FAQ with Index ${questionIndex} was already answered or not found.`);
    }
  }
}

const streamManager = new StreamInteractionManager(); 
streamManager.initiatePoll("Who is your favorite character?", ["Character A", "Character B", "Character C"]);
streamManager.postQuestion("How to implement feature X in language Y?");