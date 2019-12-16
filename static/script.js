const { Button, Notification, Grid, Message, Icon, Row, Col, Divider, Container, Panel, Header, Content, FormControl, ControlLabel, HelpBlock, Navbar, Nav, Table, Form, FormGroup, Tag } = rsuite;
const { Column, HeaderCell, Cell } = Table;

class MainContent extends React.Component {

  constructor(props) {
    super(props);
    this.state = { data: [] };
    this.fresh = this.fresh.bind(this);
    this.handleCreate = this.handleCreate.bind(this);
    this.handleDelete = this.handleDelete.bind(this);
    this.handleUpdate = this.handleUpdate.bind(this);
  }

  componentDidMount() {
    this.fresh();
  }

  handleUpdate(event) {
    const { fresh } = this;
    return function () {
      const newEvent = { uid: event.uid, content: event.content, finished: !event.finished };
      axios.put('/events/', newEvent).then(() => { fresh() });
    }
  }

  handleDelete(event) {
    const { fresh } = this;
    return function () {
      const body = { uid: event.uid };
      axios({
        url: '/events/',
        method: 'delete',
        data: body
      }).then(() => { fresh() });
    }
  }

  handleCreate() {
    const body = this.form.getFormValue();
    if (body.content && body.content !== '') {
      axios.post('/events/', body).then(() => {
        body.content = '';
        this.fresh()
      })
    } else {
      Notification.error({ title: 'Empty Content' });
    }
  }

  fresh() {
    axios.get('/events/').then(resp => {
      const dateFmt = 'YYYY-MM-DD HH:mm'
      const data = resp.data.map(item => {
        return {
          ...item,
          created_at: moment(item.created_at).format(dateFmt),
          updated_at: moment(item.updated_at).format(dateFmt),
        };
      });
      this.setState({ data });
    })
  }


  render() {
    const { data } = this.state;
    return (
      <div className="navbar-page">
        <Container>
          <Header>
            <Navbar appearance="inverse">
              <Navbar.Body>
                <Nav>
                  <Nav.Item icon={<Icon icon="home" />}>Home</Nav.Item>
                </Nav>
              </Navbar.Body>
            </Navbar>
          </Header>
          <Content>
            <Panel header="Actix-Todo" bordered >
              <p>Powered By Actix & React</p>
            </Panel>
            <Table data={data} autoHeight={true}>
              <Column resizable={true}>
                <HeaderCell>Content</HeaderCell>
                <Cell dataKey="content" />
              </Column>
              <Column flexGrow={1} >
                <HeaderCell>Create At</HeaderCell>
                <Cell dataKey="created_at"></Cell>
              </Column>
              <Column flexGrow={1} >
                <HeaderCell>Update At</HeaderCell>
                <Cell dataKey="updated_at"></Cell>
              </Column>
              <Column >
                <HeaderCell>Status</HeaderCell>
                <Cell>
                  {rowData => {
                    const { finished } = rowData;
                    if (!finished) {
                      return <Icon icon="square-o" />
                    }
                    return <Icon icon="check-square-o" />
                  }}
                </Cell>
              </Column>
              <Column flexGrow={1} >
                <HeaderCell>Actions</HeaderCell>
                <Cell>
                  {rowData => {
                    const content = rowData.finished ? 'Redo' : 'Done';
                    return (<div>
                      <a onClick={this.handleUpdate(rowData)}>{content}</a>
                      <Divider vertical />
                      <a onClick={this.handleDelete(rowData)}><Icon icon='trash' style={{ color: 'red' }} /></a>
                    </div>)
                  }}
                </Cell>
              </Column>
            </Table>
            <Divider />
            <Grid>
              <Row>
                <Col md={15} mdOffset={9}>
                  <Form layout="inline" ref={ref => this.form = ref}>
                    <FormGroup>
                      <ControlLabel><Tag>Content</Tag></ControlLabel>
                      <FormControl name="content" required />
                      <HelpBlock tooltip>todo event content</HelpBlock>
                    </FormGroup>
                    <Button appearance="primary" onClick={this.handleCreate}>Submit</Button>
                  </Form>
                </Col>
              </Row>
            </Grid>
          </Content>
        </Container>
      </div>
    )
  }
}

ReactDOM.render(<MainContent />, document.getElementById('app'));
