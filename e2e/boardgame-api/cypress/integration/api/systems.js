/// <reference types="cypress" />

context('ping', () => {
  it('pongを返す', () => {
    cy.request({
      url: 'localhost:21001/v1/systems/ping',
      method: 'GET',
    })
    .then((response) => {
      expect(response).property('status').to.equal(200)
      expect(response).property('body').property('ping').to.eq('pong')
    })
  })
})
