package main

//import (
//	"fmt"
//	"reflect"
//)
//
////	type JoinGameSessionCommand struct {
////		participantId string
////		answer        string
////	}
////
////	func (aqc *JoinGameSessionCommand) HandleCommand() {
////		// Do Apply business logic
////		// and do your dirty work on the model
////		// This will be an atomic, strongly consistent function
////		// with writes to both write model and read model
////		// using mutex locks, but consider using channels
////	}
//type CommandHandler interface {
//	Handle(CommandHandler)
//}
//
//type AnswerQuestionCommand struct {
//	participantId string
//	answer        string
//}
//
//func (aqc *AnswerQuestionCommand) Handle(command CommandHandler) {
//
//	// Apply business logic
//	// and do your dirty work on the model
//	// This will be an atomic, strongly consistent function
//	// with writes to both write model and read model
//	// using mutex locks, but consider using channels
//	fmt.Printf("Answer Question Command Reached. participantId: %v, answer: %v",
//		aqc.participantId,
//		aqc.answer)
//}
//
//type MessageHandler interface {
//	HandleMessage(message interface{})
//}
//
//type MessageRegistry struct {
//	handlers map[reflect.Type]MessageHandler
//}
//
//func (mr *MessageRegistry) Register(messageType reflect.Type, handler MessageHandler) {
//	mr.handlers[messageType] = handler
//}
//
//type CommandConverter interface {
//	Convert(payload interface{}) CommandHandler
//}
//
//type AnswerQuestionConverter struct{}
//
//type AnswerQuestionProto struct {
//	participantId string
//	answer        string
//}
//
//func (aqc *AnswerQuestionConverter) Convert(payload interface{}) CommandHandler {
//	commandPayload := payload.(*AnswerQuestionProto)
//	return &AnswerQuestionCommand{
//		participantId: commandPayload.participantId,
//		answer:        commandPayload.answer,
//	}
//}
//
//func (cmh *CommandMessageHandler) HandleMessage(message interface{}) {
//	commandMessageProto := message.(*CommandMessageProto)
//	converter := cmh.commandConverters.converters[reflect.TypeOf(commandMessageProto)]
//	command := converter.Convert(commandMessageProto)
//	cmh.dispatcher.Dispatch(command)
//}
//
//type CommandHandlerRegistry struct {
//	handlers map[reflect.Type]CommandHandler
//}
//
//func (chr *CommandHandlerRegistry) Register(commandType reflect.Type, command CommandHandler) {
//	chr.handlers[commandType] = command
//}
//
//func (chr *CommandHandlerRegistry) Dispatch(command CommandHandler) {
//	handler := chr.handlers[reflect.TypeOf(command)]
//	handler.Handle(command)
//}
//
//type CommandConverterRegistry struct {
//	converters map[reflect.Type]CommandConverter
//}
//
//func (chr *CommandConverterRegistry) Register(commandType reflect.Type, converter CommandConverter) {
//	chr.converters[commandType] = converter
//}
//
//type CommandMessageHandler struct {
//	dispatcher        CommandHandlerRegistry
//	commandConverters CommandConverterRegistry
//}
//
//type CommandMessageProto struct{}
//
//func initializeGameApp() *MessageRegistry {
//	dispatcher := &CommandHandlerRegistry{}
//	dispatcher.Register(reflect.TypeOf(AnswerQuestionCommand{}), &AnswerQuestionCommand{})
//	commandMessageConverter := &CommandConverterRegistry{}
//	commandMessageConverter.Register(reflect.TypeOf(AnswerQuestionProto{}), &AnswerQuestionConverter{})
//
//	commandMessageHandler := &CommandMessageHandler{
//		commandConverters: *commandMessageConverter,
//		dispatcher:        *dispatcher,
//	}
//
//	messageRegistry := &MessageRegistry{handlers: make(map[reflect.Type]MessageHandler)}
//	messageRegistry.Register(reflect.TypeOf(CommandMessageProto{}), commandMessageHandler)
//	return messageRegistry
//}
//
//func (mr *MessageRegistry) HandleClientMessage(envelope interface{}) error {
//	message, exists := mr.handlers[reflect.TypeOf(envelope)]
//	if !exists {
//		return fmt.Errorf("no converter registered for command type: %s", reflect.TypeOf(envelope))
//	}
//	message.HandleMessage(envelope)
//	return nil
//}
//
//func main() {
//	mr := initializeGameApp()
//	mr.HandleClientMessage(CommandMessageProto{})
//}
//
//// MIMICKING THIS
////func initializeGameApp() (*MessageRegistry, *CommandRegistry) {
////    // Create the CommandRegistry
////    commandRegistry := NewCommandRegistry()
////
////    // Create the MessageRegistry
////    messageRegistry := NewMessageRegistry(commandRegistry)
////
////    // Register Command Handlers
////    playerMoveHandler := NewPlayerMoveHandler()
////    commandRegistry.Register("PlayerMoveCommand", playerMoveHandler)
////
////    // Register Message Converters
////    playerMoveConverter := &PlayerMoveConverter{}
////    messageRegistry.Register("*PlayerMoveProto", playerMoveConverter)
////
////    return messageRegistry, commandRegistry
////}
//
////type CommandMessageHandler struct {
////	//commandHandlers   map[reflect.Type]CommandHandler
////	commandConverters map[reflect.Type]CommandConverter
////}
//
////type CommandEnvelope struct {
////	Command Command
////}
////
////type CommandRegistry struct {
////	handlers map[string]CommandHandler
////}
////
////func (r *CommandRegistry) Register(commandType string, handler CommandHandler) {
////	r.handlers[commandType] = handler
////}
////
////func (r *CommandRegistry) Dispatch(cmd Command) error {
////	handler, exists := r.handlers[cmd.Type()]
////	if !exists {
////		return fmt.Errorf("no handler for command type: %s", cmd.Type())
////	}
////	return handler.Handle(cmd)
////}
////
////func initializeGameApp() (*CommandRegistry, *EventRegistry, *PlayerSessionRegistry) {
////	// Step 1: Create registries
////	commandRegistry := &CommandRegistry{handlers: make(map[string]CommandHandler)}
////	eventRegistry := &EventRegistry{subscribers: make(map[string][]EventSubscriber)}
////	sessionRegistry := &PlayerSessionRegistry{sessions: make(map[string]*PlayerSession)}
////
////	// Step 2: Create dependencies (e.g., services, repositories)
////	gameRepository := NewInMemoryGameRepository()     // Or a database-backed repository
////	readModelUpdater := NewInMemoryReadModelUpdater() // Updates the in-memory read model
////	broadcastService := NewBroadcastService(sessionRegistry)
////
////	// Step 3: Register command handlers
////	playerMoveHandler := NewPlayerMoveHandler(gameRepository, eventRegistry)
////	commandRegistry.Register("PlayerMoveCommand", playerMoveHandler)
////
////	// Step 4: Register event subscribers
////	eventRegistry.Register("PlayerMovedEvent", readModelUpdater)
////	eventRegistry.Register("PlayerMovedEvent", broadcastService)
////
////	// Step 5: Return the registries
////	return commandRegistry, eventRegistry, sessionRegistry
////}
////
////func main() {
////	// Initialize the application
////	commandRegistry, eventRegistry, sessionRegistry := initializeGameApp()
////
////	// WebSocket server setup
////	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
////		conn, err := upgrader.Upgrade(w, r, nil)
////		if err != nil {
////			log.Println("WebSocket upgrade failed:", err)
////			return
////		}
////
////		playerID := r.URL.Query().Get("playerId")
////		if playerID == "" {
////			log.Println("Player ID missing")
////			conn.Close()
////			return
////		}
////
////		go handleWebSocket(conn, commandRegistry, sessionRegistry, playerID)
////	})
////
////	log.Println("Starting WebSocket server on :8080")
////	log.Fatal(http.ListenAndServe(":8080", nil))
////
////}
////
