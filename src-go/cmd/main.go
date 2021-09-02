package main

import (
	"context"
	"log"
	"math/rand"
	"net"
	"time"

	"github.com/brianvoe/gofakeit"
	"github.com/neksuhs/grpctesting/pb"
	"google.golang.org/grpc"
)

const (
	port = ":50051"
)

// server is used to implement user.UsersServer.
type server struct {
	pb.UnimplementedUsersServer
}

// AllUsers implements user.UsersServer
func (s *server) AllUsers(ctx context.Context, in *pb.AllUsersRequest) (*pb.AllUsersReply, error) {
	min := 20
	max := 60
	start := time.Now()

	var users []*pb.User
	for i := 0; i < 200; i++ {
		gofakeit.Seed(0)
		user := pb.User{
			FirstName: gofakeit.FirstName(),
			LastName:  gofakeit.LastName(),
			Username:  "Username",
			Gender:    gofakeit.Gender(),
			Age:       uint32(rand.Intn(max-min) + min),
			Email:     gofakeit.Email(),
			Tel:       gofakeit.Phone(),
		}
		users = append(users, &user)
	}
	reply := pb.AllUsersReply{
		GoTime: uint64(time.Since(start).Microseconds()),
		Users:  users,
	}
	return &reply, nil
}

// // AllUsers implements user.UsersServer
// func (s *server) AllUsers(ctx context.Context, in *pb.AllUsersRequest) (*pb.AllUsersReply, error) {
// 	// min := 20
// 	// max := 60
// 	start := time.Now()

// 	var users []*pb.User

// 	user := pb.User{
// 		FirstName: "FirstName",
// 		LastName:  "LastName",
// 		Username:  "Username",
// 		Gender:    "Male",
// 		Age:       40,
// 		Email:     "email@email.com",
// 		Tel:       "5555555555",
// 	}

// 	for i := 0; i < 200; i++ {
// 		users = append(users, &user)
// 	}
// 	reply := pb.AllUsersReply{
// 		GoTime: uint64(time.Since(start).Microseconds()),
// 		Users:  users,
// 	}
// 	return &reply, nil
// }

func main() {
	lis, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterUsersServer(s, &server{})
	log.Printf("gRPC server listening at: %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
