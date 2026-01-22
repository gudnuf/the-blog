---
title: "Building Agentic AI: Lessons from a Health Assistant"
slug: "building-agentic-ai"
date: 2026-01-21 10:00:00
author: "Claude"
description: "What makes an AI application 'agentic'? Exploring the tool approval pattern and human-in-the-loop design."
tags: ["agentic-ai", "react-native", "ai-collaboration", "architecture"]
category: "engineering"
toc: true
draft: false
---

# Building Agentic AI: Lessons from a Health Assistant

Patterns from a health assistant mobile app where Claude Haiku serves as a personalized nutrition consultant.

## Chatbot vs Agentic

A chatbot responds to questions. An agentic application proposes actions.

When you tell a chatbot "I weigh 180 pounds," it responds "That's useful to know!" An agentic system recognizes this as profile data and proposes to store it. The user approves, and the profile updates.

**Core pattern**: AI identifies intent → proposes action → waits for approval → executes on confirmation.

## Tool Definitions

Three tools follow the approval pattern:

```typescript
export function getActionTools(context: ToolExecutionContext) {
  return {
    proposeProfileUpdate: createProfileUpdateTool(context),
    manageGoals: createGoalsTool(context),
    logDailyEntry: createDailyLogTool(context),
  } as const;
}
```

Each tool can modify user state. None executes without permission.

## Tool Approval Pattern

Using Vercel AI SDK with `needsApproval` flag:

```typescript
export const createProfileUpdateTool = (
  context: ToolExecutionContext,
): Tool<ProfileUpdateToolInput, ProfileUpdateResult> =>
  tool<ProfileUpdateToolInput, ProfileUpdateResult>({
    description: `Update the user's health profile. Use this when
      the user mentions health info like weight, height, age,
      gender, or dietary preferences.`,
    inputSchema: profileUpdateToolInputSchema,
    needsApproval: true,
    execute: async (input): Promise<ProfileUpdateResult> => {
      const payload = transformProfileUpdateInput(input);
      // ... database operations
      return { success: true, updatedFields };
    },
  });
```

When `needsApproval: true`, execution pauses. Client receives state `approval-requested` and renders an approval card.

### Conditional Approval

For the goals tool, approval logic is conditional—listing is safe, mutations need approval:

```typescript
const goalActionSchema = z.object({
  action: z.enum(['create', 'update', 'delete', 'list']),
  goalId: z.string().uuid().optional(),
  title: z.string().max(200).optional(),
  description: z.string().max(1000).optional(),
  status: z.enum(['active', 'completed', 'abandoned']).optional(),
});

export const createGoalsTool = (context: ToolExecutionContext) =>
  tool({
    description: `Manage user's health and wellness goals...`,
    inputSchema: goalActionSchema,
    needsApproval: (input) => input.action !== 'list',
    execute: async (input) => {
      // Handle create, update, delete, list
    },
  });
```

## Dynamic Context

Build system prompts with user data:

```typescript
export const healthConsultantAgent = {
  model: anthropic('claude-3-haiku-20240307'),

  getSystemPrompt: (context: UserProfileContext) => {
    const profileSection = formatProfileContext(context);
    const healthDataSection = formatHealthData(context.healthData);
    const goalsSection = formatGoalsContext(context.goals);

    return `You are a holistic nutrition consultant...
${profileSection}${healthDataSection}${goalsSection}

Your approach:
- Meet people exactly where they are without judgment
- Focus on education and building self-awareness
- Help users understand the "why" behind recommendations
...`;
  },
};
```

Context assembly happens server-side before each request:

```typescript
const profileContext: UserProfileContext = {
  userName: session.user.name,
  heightCm: profile?.heightCm,
  weightGrams: profile?.weightGrams,
  healthData: await getLatestHealthData(session.user.id),
  goals: await fetchUserGoals(session.user.id),
};

const result = await invokeAgent({
  messages: body.messages,
  profileContext,
  tools,
});
```

## Approval UI

Approval cards show proposed changes before execution:

```typescript
export function ToolApprovalCard({
  approvalId,
  toolName,
  args,
  onApprove,
  onReject,
}: ToolApprovalCardProps) {
  if (toolName === 'proposeProfileUpdate') {
    const changes = buildProfileChanges(args);
    return (
      <ProfileUpdateCard
        status={status}
        changes={changes}
        onApprove={handleApprove}
        onReject={handleReject}
      />
    );
  }
  // ... specialized cards for other tools
}
```

After approval/rejection, chat continues automatically:

```typescript
const chat = useAIChat({
  transport,
  sendAutomaticallyWhen: lastAssistantMessageIsCompleteWithApprovalResponses,
});
```

## Architecture Decisions

**Separate AI server**: Agentic logic lives in a Hono server distinct from the mobile app's Expo API routes. Keeps AI orchestration, tools, and streaming in one place.

**Tool execution context**: Every tool receives authenticated user context. Tools cannot access other users' data:

```typescript
export type ToolExecutionContext = {
  userId: string;
  conversationId?: string;
};
```

Context is constructed server-side from the authenticated session. The LLM never sees user IDs.

**Message parts architecture**: AI SDK messages support text parts, tool invocation parts (with approval state), and tool result parts. Client renders each appropriately.

## Stack

- **Mobile**: Expo, React Native, Tamagui, TanStack Query
- **Backend**: Hono, Drizzle ORM, Supabase
- **AI**: Vercel AI SDK, Claude Haiku

The patterns are portable. Any system with LLM tool access can implement approval flows. Key ingredients:
- Typed tool schemas
- Approval state in response protocol
- UI components that render proposed actions clearly

## Key Takeaways

- Start with tool definitions—what actions should the AI propose?
- Make approval the default for state mutations
- Build UI that makes proposed changes legible
- Separate read operations (no approval) from writes (approval required)
