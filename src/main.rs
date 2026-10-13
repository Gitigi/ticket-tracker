use std::sync::Arc;

use ticket_tracker::{
    adapter::persistence::MemoryPersistance,
    application::{TicketUseCases, dto::CreateTicket},
    domain,
};

#[tokio::main]
async fn main() {
    let memory = MemoryPersistance::new();
    let use_case = TicketUseCases::new(Arc::new(memory));
    println!(
        "tickets {:?}",
        use_case.list_tickets(Default::default()).await
    );
    let ticket = use_case
        .create(CreateTicket {
            title: "Assignment".into(),
            description: Some("Finish my assignment".into()),
            priority: domain::TicketPriority::Low,
        })
        .await
        .unwrap();
    println!(
        "tickets {:?}",
        use_case.list_tickets(Default::default()).await
    );

    use_case
        .update_priority(ticket.id, domain::TicketPriority::High)
        .await
        .unwrap();

    println!(
        "tickets {:?}",
        use_case.list_tickets(Default::default()).await
    );
}
